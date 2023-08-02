// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			supabase: SupabaseClient<Database>
			getSession(): Promise<Session | null>
		  }
		  interface PageData {
			session: Session | null
		  }
		// interface Platform {}
	}
	// Your selected Skeleton theme:
}

export {};
