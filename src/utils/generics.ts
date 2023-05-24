import { invoke } from "@tauri-apps/api";

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
