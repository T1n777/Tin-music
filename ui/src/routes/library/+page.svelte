<script module lang="ts">
	// Module scope, so returning to the library (back from an album you opened, or via the sidebar)
	// keeps the tab you were on instead of snapping to All.
	let lastTab = 'all';
</script>

<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { page } from '$app/state';
	import { HugeiconsIcon } from '@hugeicons/svelte';
	import {
		Add01Icon,
		DriveIcon,
		MusicNote01Icon,
		MusicNoteSquare02Icon,
		Playlist02Icon,
		SquareStackIcon,
		UserCheck01Icon,
		UserSharingIcon
	} from '@hugeicons/core-free-icons';
	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import LibrarySongs from '$lib/components/LibrarySongs.svelte';
	import LocalMusic from '$lib/components/LocalMusic.svelte';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import MediaCardSkeleton from '$lib/components/MediaCardSkeleton.svelte';
	import ErrorState from '$lib/components/ErrorState.svelte';
	import type { BrowseItem } from '$lib/api';
	import {
		auth,
		personal,
		toast,
		library,
		loadLibrary,
		loadLibraryExtras,
		openNewPlaylist
	} from '$lib/player.svelte';
	import { mergeSaved } from '$lib/personal';
	import { reveal } from '$lib/reveal.svelte';
	import { t } from '$lib/i18n.svelte';

	// `?tab=local` so anything that sends you back here (an album whose files were deleted) lands
	// on the tab you came from instead of a sign-in prompt.
	let tab = $state(page.url.searchParams.get('tab') ?? lastTab);
	$effect(() => {
		lastTab = tab;
	});
	// Artists splits like YouTube Music's: the artists behind your songs, or the ones you subscribe
	// to. Only the second can be taken out of the library (an unsubscribe); the first leaves with
	// its songs.
	let artistTab = $state('artists');

	// Everything here lives in the shared `library` store, so a revisit renders the cached grid
	// immediately and the forced refresh below swaps in fresh data behind it. What was saved on this
	// machine merges in per tab (`mergeSaved`), which is the whole library when signed out.
	const playlists = $derived(mergeSaved(personal, library.items, 'playlist'));
	const albums = $derived(mergeSaved(personal, library.albums, 'album'));
	const artists = $derived(mergeSaved(personal, library.artists, 'artist'));
	const subscriptions = $derived(mergeSaved(personal, library.subscriptions, 'artist'));
	const all = $derived([...playlists, ...albums, ...artists]);
	// One per tab rather than one shared instance reset on switch: an `$effect` reset lands
	// after the render it is meant to govern, so switching tabs would build the new tab's grid
	// against the old tab's count and immediately tear the excess back down. A tab keeping its
	// own depth also means coming back to one lands where you left it.
	const rvAll = reveal();
	const rvPlaylists = reveal();
	const rvAlbums = reveal();
	const rvArtists = reveal();
	const rvSubscriptions = reveal();
	const loading = $derived((library.loading || library.extrasLoading) && !all.length);
	const error = $derived(library.error ?? library.extrasError);
	// Only the empty states differ: signed out there is no account library to be missing yet.
	const signedOut = $derived(!auth.account?.signedIn);
	onMount(load);

	function load() {
		loadLibrary(true);
		loadLibraryExtras(true);
	}
</script>

{#snippet grid(items: BrowseItem[], empty: string, rv: ReturnType<typeof reveal>)}
	{#if items.length}
		<div class="card-grid content-in">
			{#each items.slice(0, rv.count(items.length)) as item (item.kind + item.id)}
				<MediaCard {item} />
			{/each}
		</div>
		<!-- Outside the grid, or it would be laid out as a cell. -->
		{#if rv.more(items.length)}<div {@attach rv.sentinel}></div>{/if}
	{:else}
		<p class="text-sm text-muted-foreground">{empty}</p>
	{/if}
{/snippet}

<div class="p-6">
	<div class="mb-6 flex items-center justify-between">
		<h1 class="font-heading text-2xl font-bold">{t('library.title')}</h1>
		<div class="flex items-center gap-2">
			<!-- Signed out too: a playlist can live on this machine with no account (#251). -->
			<Button variant="outline" size="sm" class="gap-2" onclick={() => openNewPlaylist()}>
				<HugeiconsIcon icon={Add01Icon} class="h-4 w-4" /> {t('nav.new_playlist')}
			</Button>
		</div>
	</div>


	<!-- The tabs always render: Local music needs neither an account nor a connection. -->
	<Tabs.Root bind:value={tab}>
		<Tabs.List class="mb-4">
			<Tabs.Trigger value="all">
				<HugeiconsIcon icon={SquareStackIcon} class="h-4 w-4" /> {t('common.all')}
			</Tabs.Trigger>
			<Tabs.Trigger value="playlists">
				<HugeiconsIcon icon={Playlist02Icon} class="h-4 w-4" /> {t('library.playlists_tab')}
			</Tabs.Trigger>
			<Tabs.Trigger value="albums">
				<HugeiconsIcon icon={MusicNoteSquare02Icon} class="h-4 w-4" /> {t('library.albums_tab')}
			</Tabs.Trigger>
			<Tabs.Trigger value="artists">
				<HugeiconsIcon icon={UserSharingIcon} class="h-4 w-4" /> {t('library.artists_tab')}
			</Tabs.Trigger>
			<Tabs.Trigger value="songs">
				<HugeiconsIcon icon={MusicNote01Icon} class="h-4 w-4" /> {t('library.songs_tab')}
			</Tabs.Trigger>
			<Tabs.Trigger value="local">
				<HugeiconsIcon icon={DriveIcon} class="h-4 w-4" /> {t('library.local_tab')}
			</Tabs.Trigger>
		</Tabs.List>
		<!-- Every branch below is gated on `tab`, because bits-ui never unmounts an inactive panel: it
		     renders every one and hides the inactive ones. Left alone, opening Library builds each card twice
		     (once for All, once for its own tab) and mounts the whole Local tab, disk scan included,
		     for a panel you cannot see. -->
		<!-- Songs and Local stand apart: one is a track list rather than card grid, the
		     other needs neither an account nor a connection, and the states below fit none of them. -->
		<Tabs.Content value="songs">
			{#if tab === 'songs'}
				{#if signedOut}
					<p class="text-sm text-muted-foreground">{t('library.songs_signed_out')}</p>
				{:else}
					<LibrarySongs />
				{/if}
			{/if}
		</Tabs.Content>
		<Tabs.Content value="local">{#if tab === 'local'}<LocalMusic />{/if}</Tabs.Content>
		{#if tab === 'local' || tab === 'songs'}
			<!-- nothing else: the grid states below have no bearing on these three -->
		{:else if loading}
			<div class="card-grid">
				{#each Array(12) as _, i (i)}
					<MediaCardSkeleton />
				{/each}
			</div>
		{:else if error && !all.length}
			<!-- Only when there is nothing to fall back on. Now that the grid is cached across visits, a
			     refresh that fails should leave the library you were looking at on screen. -->
			<ErrorState message={error} onRetry={load} />
		{:else}
			<Tabs.Content value="all">
				{#if tab === 'all'}
					{@render grid(
						all,
						signedOut ? t('library.empty_signed_out') : t('library.empty'),
						rvAll
					)}
				{/if}
			</Tabs.Content>
			<Tabs.Content value="playlists">
				{#if tab === 'playlists'}
					{@render grid(
						playlists,
						t('library.no_saved_playlists'),
						rvPlaylists
					)}
				{/if}
			</Tabs.Content>
			<Tabs.Content value="albums">
				{#if tab === 'albums'}
					{@render grid(
						albums,
						t('library.no_saved_albums'),
						rvAlbums
					)}
				{/if}
			</Tabs.Content>
			<Tabs.Content value="artists">
				{#if tab === 'artists' && signedOut}
					{@render grid(artists, t('library.no_saved_artists'), rvArtists)}
				{:else if tab === 'artists'}
					<!-- `line`, same as Uploads: a second pill row would read as the first drawn twice. -->
					<Tabs.Root bind:value={artistTab}>
						<Tabs.List variant="line" class="mb-4">
							<Tabs.Trigger value="artists">
								<HugeiconsIcon icon={UserSharingIcon} class="h-4 w-4" /> {t('library.artists_tab')}
							</Tabs.Trigger>
							<Tabs.Trigger value="subscriptions">
								<HugeiconsIcon icon={UserCheck01Icon} class="h-4 w-4" /> {t('library.subscriptions_tab')}
							</Tabs.Trigger>
						</Tabs.List>
						<Tabs.Content value="artists">
							{#if artistTab === 'artists'}
								<!-- Unmerged: what was saved here is a subscription, and lists under that. -->
								{@render grid(library.artists, t('library.no_artists'), rvArtists)}
							{/if}
						</Tabs.Content>
						<Tabs.Content value="subscriptions">
							{#if artistTab === 'subscriptions'}
								{@render grid(subscriptions, t('library.no_subscriptions'), rvSubscriptions)}
							{/if}
						</Tabs.Content>
					</Tabs.Root>
				{/if}
			</Tabs.Content>
		{/if}
	</Tabs.Root>
</div>
