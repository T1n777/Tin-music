<script lang="ts">
	import { HugeiconsIcon } from '@hugeicons/svelte';
	import { ServerIcon, ArrowRight01Icon } from '@hugeicons/core-free-icons';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as api from '$lib/api';
	import { auth, loadLibrary, loadLibraryExtras, refreshView, toast, ui } from '$lib/player.svelte';

	let url = $state('');
	let username = $state('');
	let password = $state('');
	let connecting = $state(false);
	let error = $state<string | null>(null);

	const close = () => {
		ui.serverLoginOpen = false;
		error = null;
	};

	async function connect() {
		const cleanUrl = url.trim();
		const cleanUser = username.trim();
		if (!cleanUrl) {
			error = 'Please enter your Navidrome server URL';
			return;
		}
		if (!cleanUser) {
			error = 'Please enter your username';
			return;
		}

		connecting = true;
		error = null;
		try {
			const info = await api.navidromeConnect(cleanUrl, cleanUser, password);
			toast.success(`Connected to Navidrome ${info.version ? `(v${info.version})` : ''}`);
			auth.account = await api.getAccount();
			close();
			refreshView();
			loadLibrary(true);
			loadLibraryExtras(true);
		} catch (e) {
			error = String(e);
			toast.error(`Connection failed: ${e}`);
		} finally {
			connecting = false;
		}
	}
</script>

<Dialog.Root bind:open={ui.serverLoginOpen}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 text-primary">
					<HugeiconsIcon icon={ServerIcon} class="h-5 w-5" />
				</div>
				<div>
					<Dialog.Title class="text-base font-semibold">Connect to Navidrome</Dialog.Title>
					<Dialog.Description class="text-xs text-muted-foreground">
						Enter your Navidrome Subsonic server details to stream your music library.
					</Dialog.Description>
				</div>
			</div>
		</Dialog.Header>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				connect();
			}}
			class="space-y-4 py-2"
		>
			<div class="space-y-1.5">
				<label for="navidrome-url" class="text-xs font-medium text-foreground">Server URL</label>
				<Input
					id="navidrome-url"
					type="url"
					placeholder="http://localhost:4533 or https://music.example.com"
					bind:value={url}
					disabled={connecting}
					required
					autocomplete="url"
				/>
			</div>

			<div class="space-y-1.5">
				<label for="navidrome-username" class="text-xs font-medium text-foreground">Username</label>
				<Input
					id="navidrome-username"
					type="text"
					placeholder="admin"
					bind:value={username}
					disabled={connecting}
					required
					autocomplete="username"
				/>
			</div>

			<div class="space-y-1.5">
				<label for="navidrome-password" class="text-xs font-medium text-foreground">Password</label>
				<Input
					id="navidrome-password"
					type="password"
					placeholder="••••••••"
					bind:value={password}
					disabled={connecting}
					autocomplete="current-password"
				/>
			</div>

			{#if error}
				<div class="rounded-lg bg-destructive/10 p-2.5 text-xs text-destructive">
					{error}
				</div>
			{/if}

			<Dialog.Footer class="gap-2 sm:gap-0 pt-2">
				<Button type="button" variant="outline" onclick={close} disabled={connecting}>
					Cancel
				</Button>
				<Button type="submit" disabled={connecting} class="gap-1.5">
					{#if connecting}
						Connecting...
					{:else}
						Connect
						<HugeiconsIcon icon={ArrowRight01Icon} class="h-4 w-4" />
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
