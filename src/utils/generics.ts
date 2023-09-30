import { invoke } from "@tauri-apps/api";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";
import type { Options } from "@tauri-apps/api/notification";

export async function parseTauriCommand<T>(
  command: string,
  args?: { [key: string]: any }
): Promise<T> {
  try {
    return JSON.parse(JSON.stringify(await invoke(command, args)));
  } catch (error) {
    console.error(error);
  }
}

export async function tauriNotify(notificationOptions: Options) {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }
  if (permissionGranted) {
    sendNotification(notificationOptions);
  }
}
