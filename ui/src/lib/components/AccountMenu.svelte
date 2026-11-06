<script lang="ts">
	// Account control for the titlebar (context/15) — moved out of the sidebar so sign-in lives in the
	// top bar. Its own component because Titlebar.svelte already uses a single shared mx/my/menuOpen
	// for the Last.fm menu; a second menu in that file would fight over them.
	// Multi-account: below the active account the menu lists the *other* saved Google accounts, one
	// click each to switch to, plus Add account (Google's AddSession flow) and per-account removal.
	import { HugeiconsIcon } from '@hugeicons/svelte';
	import {
		UserCircleIcon,
		Logout01Icon,
		ArrowDown01Icon,
		Add01Icon,
		Cancel01Icon,
		UserMinus01Icon,
		ServerIcon
	} from '@hugeicons/core-free-icons';
	import { Button } from '$lib/components/ui/button';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as api from '$lib/api';
	import { auth, openChannelPicker, openServerLogin, toast } from '$lib/player.svelte';
	import { thumb } from '$lib/thumb';
	import { anchorMenu, fitMenu, NO_ANCHOR } from '$lib/menu';
	import { t } from '$lib/i18n.svelte';

	let menuOpen = $state(false);
	let anchor = $state(NO_ANCHOR);
	let accounts = $state<api.SavedAccount[]>([]);
	let busy = $state(false);
	// The account the confirm dialog is asking about. Deliberately not cleared when the dialog
	// closes, so its name doesn't blank out mid close-animation; `confirmOpen` is the real state.
	let removing = $state<api.SavedAccount | null>(null);
	let confirmOpen = $state(false);
	// The active account already has the block above it; listing it again just repeats itself.
	const others = $derived(accounts.filter((a) => !a.active));

	/**
	 * Right-anchored under the trigger, like the Last.fm menu next to it. The saved-account list
	 * is refetched on every open so it can't go stale while the menu was closed.
	 */
	async function openMenu(e: MouseEvent) {
		anchor = anchorMenu(e, { align: 'right' });
		menuOpen = !menuOpen;
		if (menuOpen) await loadAccounts();
	}

	/** Fetch the saved accounts for the menu; on failure show a toast and an empty list. */
	async function loadAccounts() {
		try {
			accounts = await api.getGoogleAccounts();
		} catch (e) {
			toast.error(String(e));
			accounts = [];
		}
	}

	function openConnect() {
		menuOpen = false;
		openServerLogin();
	}

	async function scanLibrary() {
		menuOpen = false;
		try {
			await api.navidromeScanLibrary();
			toast.success('Navidrome library scan started');
		} catch (e) {
			toast.error(`Scan failed: ${e}`);
		}
	}

	// Sign-in/out state arrives via the `auth-changed` event (player.svelte.ts), which also reloads
	// the library and remounts the page — nothing to assign here.
	async function doSignOut() {
		menuOpen = false;
		await api.signOut();
	}

	function signInGoogle() {
		api.loginWebview(); // native sign-in window takes over; result arrives via auth-changed
		menuOpen = false;
	}

	/**
	 * Add a Google account. Signs the login webview out of Google first (Rust side) so the fresh
	 * sign-in lands on the account the user actually enters, which costs a password prompt.
	 */
	function addAccount() {
		api.loginWebview(true);
		menuOpen = false;
	}

	function switchChannel() {
		menuOpen = false;
		openChannelPicker();
	}

	/** Activate a saved account. `auth-changed` fires on success and the library reload follows. */
	async function chooseAccount(account: api.SavedAccount) {
		if (busy) return;
		busy = true;
		try {
			await api.switchGoogleAccount(account.id);
			menuOpen = false;
		} catch (e) {
			toast.error(String(e));
		} finally {
			busy = false;
		}
	}

	/** Ask before removing: the row's cookies are deleted and only a fresh sign-in brings them back. */
	function askRemove(account: api.SavedAccount) {
		if (busy) return;
		removing = account;
		confirmOpen = true;
	}

	/** Delete a saved account. Only inactive ones are listed, so this never signs the user out. */
	async function removeAccount() {
		const account = removing;
		if (!account) return;
		// bits-ui's Action is a plain button (only Cancel closes the dialog), so close it here.
		// Before the await, not after: the row disappears at once and a failure arrives as a toast.
		confirmOpen = false;
		busy = true;
		try {
			await api.removeGoogleAccount(account.id);
			accounts = accounts.filter((a) => a.id !== account.id);
		} catch (e) {
			toast.error(String(e));
		} finally {
			busy = false;
		}
	}
</script>

<button
	onclick={auth.account?.signedIn ? openMenu : openConnect}
	title={auth.account?.signedIn ? (auth.account.name ?? t('nav.account')) : 'Connect to Navidrome'}
	aria-expanded={menuOpen}
	class="flex h-full cursor-pointer items-center gap-2 px-2.5 text-xs transition-colors hover:bg-muted aria-expanded:bg-muted"
>
	{#if auth.account?.signedIn && auth.account.thumbnail}
		<!-- max-width:none defeats Tailwind Preflight's `img{max-width:100%}`, which in a tight box
		     clamps width to the content-box while height stays fixed → a vertical oval. Inline so it's
		     immune to Preflight and to stale dev CSS. -->
		<img
			src={thumb(auth.account.thumbnail, 64)}
			alt=""
			style="width:1.25rem;height:1.25rem;max-width:none"
			class="shrink-0 rounded-full object-cover ring-1 ring-border"
		/>
	{:else}
		<HugeiconsIcon icon={ServerIcon} class="h-4 w-4 shrink-0 text-muted-foreground" />
	{/if}
	<span class="hidden max-w-28 truncate font-medium lg:block">
		{auth.account?.signedIn ? (auth.account.name ?? t('nav.account')) : 'Connect'}
	</span>
	{#if auth.account?.signedIn}
		<HugeiconsIcon
			icon={ArrowDown01Icon}
			class="hidden h-3.5 w-3.5 shrink-0 text-muted-foreground transition-transform duration-200 lg:block {menuOpen
				? 'rotate-180'
				: ''}"
		/>
	{/if}
</button>

{#if menuOpen}
	<button
		class="fixed inset-0 z-40 cursor-default"
		onclick={() => (menuOpen = false)}
		aria-label={t('common.close')}
	></button>
	<div
		class="fixed z-50 w-72 animate-in rounded-xl border bg-popover p-4 text-popover-foreground shadow-xl duration-150 fade-in-0 zoom-in-95"
		style={anchor.style}
		{@attach fitMenu(anchor)}
	>
		{#if auth.account?.signedIn}
			<div class="mb-3">
				<div class="truncate text-sm font-medium">{auth.account.name ?? t('nav.account')}</div>
				{#if auth.account.handle || auth.account.email}
					<div class="truncate text-xs text-muted-foreground">
						{auth.account.handle ?? auth.account.email}
					</div>
				{/if}
			</div>
		{/if}

		{#if others.length > 0}
			<div class="mb-2 space-y-0.5">
				{#each others as account (account.id)}
					<div class="flex items-center gap-2 rounded-md px-3 py-1 hover:bg-muted">
						<button
							type="button"
							onclick={() => chooseAccount(account)}
							disabled={busy}
							class="flex min-w-0 flex-1 cursor-pointer items-center gap-2 text-left disabled:cursor-wait"
						>
							{#if account.thumbnail}
								<img
									src={thumb(account.thumbnail, 48)}
									alt=""
									style="width:1.25rem;height:1.25rem;max-width:none"
									class="shrink-0 rounded-full object-cover ring-1 ring-border"
								/>
							{:else}
								<HugeiconsIcon
									icon={UserCircleIcon}
									class="h-5 w-5 shrink-0 text-muted-foreground"
								/>
							{/if}
							<span class="min-w-0 flex-1">
								<span class="block truncate text-xs font-medium">
									{account.name ?? t('nav.account')}
								</span>
								{#if account.handle || account.email}
									<span class="block truncate text-[11px] text-muted-foreground">
										{account.handle ?? account.email}
									</span>
								{/if}
							</span>
						</button>
						<button
							type="button"
							onclick={() => askRemove(account)}
							disabled={busy}
							title={t('nav.remove_account')}
							aria-label={t('nav.remove_account')}
							class="shrink-0 cursor-pointer rounded p-0.5 text-muted-foreground transition-colors hover:text-destructive disabled:cursor-wait"
						>
							<HugeiconsIcon icon={Cancel01Icon} class="h-3.5 w-3.5" />
						</button>
					</div>
				{/each}
			</div>
		{/if}

		{#if auth.account?.signedIn}
			<Button variant="outline" size="sm" class="mb-2 w-full justify-start gap-2" onclick={scanLibrary}>
				<HugeiconsIcon icon={ServerIcon} class="size-4 shrink-0" />
				Rescan Library
			</Button>
			<Button variant="outline" size="sm" class="mb-2 w-full justify-start gap-2" onclick={openConnect}>
				<HugeiconsIcon icon={ServerIcon} class="size-4 shrink-0" />
				Switch Server
			</Button>
			<Button variant="outline" size="sm" class="w-full justify-start gap-2" onclick={doSignOut}>
				<HugeiconsIcon icon={Logout01Icon} class="size-4 shrink-0" />
				Disconnect
			</Button>
		{:else}
			<p class="text-sm font-medium">Navidrome Server</p>
			<p class="mt-1 text-xs text-muted-foreground">
				Connect to your Navidrome music server to stream your tracks and playlists.
			</p>
			<Button class="mt-3 w-full gap-2" onclick={openConnect}>
				<HugeiconsIcon icon={ServerIcon} class="size-4" />
				Connect to Server
			</Button>
		{/if}
	</div>
{/if}

<!-- Outside the menu block on purpose: the menu can close under it (a switch, an auth-changed
     remount) and the confirm must not vanish with it. -->
<AlertDialog.Root bind:open={confirmOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<!-- Muted, not destructive red: this deletes a row from a list, and dressing it as a
			     danger reads as "delete your Google account". -->
			<AlertDialog.Media class="text-muted-foreground">
				<HugeiconsIcon icon={UserMinus01Icon} />
			</AlertDialog.Media>
			<AlertDialog.Title>{t('nav.remove_account_title')}</AlertDialog.Title>
			<AlertDialog.Description>
				{t('nav.remove_account_desc', { name: removing?.name ?? t('nav.account') })}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>{t('common.cancel')}</AlertDialog.Cancel>
			<AlertDialog.Action onclick={removeAccount}>
				{t('common.remove')}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
