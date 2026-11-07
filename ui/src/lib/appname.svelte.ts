// Custom app name chosen by user.
// Stored in SQLite database settings table (`app_name`), persisting across updates, restarts, and reinstalls.

import { getCurrentWindow } from '@tauri-apps/api/window';
import { getSettings, setSetting } from './api';

export const DEFAULT_APP_NAME = 'Tin-music';

export const appName = $state({
	current: DEFAULT_APP_NAME
});

/**
 * Load the saved app name from settings. Updates reactive state and the OS window title.
 */
export async function loadAppName(): Promise<void> {
	try {
		const settings = await getSettings();
		const saved = settings.app_name?.trim();
		appName.current = saved || DEFAULT_APP_NAME;
		await getCurrentWindow().setTitle(appName.current).catch(() => {});
	} catch (e) {
		console.error('Failed to load custom app name:', e);
		appName.current = DEFAULT_APP_NAME;
	}
}

/**
 * Set a custom app name. Pass null or empty string to reset to default 'Tin-music'.
 */
export async function setCustomAppName(name: string | null): Promise<void> {
	const trimmed = (name ?? '').trim();
	const finalName = trimmed || DEFAULT_APP_NAME;
	appName.current = finalName;
	await setSetting('app_name', trimmed);
	await getCurrentWindow().setTitle(finalName).catch(() => {});
}
