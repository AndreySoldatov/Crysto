<script lang="ts">
	import { Plus, X } from '@lucide/svelte';

	export let tags: string[];
	export let deletable: boolean = false;
</script>

<div class="flex w-full flex-wrap items-center gap-2">
	{#if !deletable}
		{#each tags as tag}
			<button class="btn h-auto rounded-full p-1 px-2 text-sm btn-soft btn-primary">{tag}</button>
		{/each}
	{:else}
		{#each tags as tag}
			<div
				class="flex flex-row items-center gap-2 rounded-full bg-primary/8 p-1 pl-3 text-sm font-semibold text-primary select-none"
			>
				{tag}
				<div class="btn btn-circle h-5 w-5 btn-ghost btn-primary"><X class="h-4" /></div>
			</div>
		{/each}
	{/if}
	<button class="btn btn-circle h-6 w-6 btn-dash btn-primary">
		<Plus class="h-4" />
	</button>
</div>
