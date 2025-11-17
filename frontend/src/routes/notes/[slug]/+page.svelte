<script lang="ts">
	import TipTap from '$lib/TipTap.svelte';
	import { Plus } from '@lucide/svelte';
	import TagList from '$lib/TagList.svelte';

	const title = '';
	const created = '20:32 12.07.2025';
	const modified = '20:45 12.07.2025';

	const tags = ['this', 'tag-list', 'test'];
</script>

<div class="mt-18 flex w-full flex-col gap-4 px-8 md:w-xl md:px-4">
	<div class="flex flex-row justify-between">
		<p class="text-sm text-base-content/40">
			Modified: {modified}
		</p>
		<p class="text-sm text-base-content/40">
			Created: {modified}
		</p>
	</div>

	<TagList {tags} deletable={true} />

	<input
		type="text"
		value={title}
		placeholder="Title"
		class="border-none bg-transparent p-0 text-4xl font-semibold caret-primary outline-none placeholder:text-base-content/30 focus:ring-0 focus:outline-none"
	/>
	<main>
		<TipTap />
	</main>
</div>
