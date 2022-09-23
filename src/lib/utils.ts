import { message } from "@tauri-apps/api/dialog";

export async function showError(body?: unknown): Promise<void> {
    console.error(body);
    await message(`${body ?? "Unknown Error"}`, { title: "Error", type: "error" })
}