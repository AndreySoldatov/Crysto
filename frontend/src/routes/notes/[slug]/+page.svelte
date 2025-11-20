<script lang="ts">
	import TipTap from '$lib/TipTap.svelte';
	import { Plus } from '@lucide/svelte';
	import TagList from '$lib/TagList.svelte';

	const title = '';
	const created = '20:32 12.07.2025';
	const modified = '20:45 12.07.2025';

	const tags = ['this', 'tag-list', 'test'];
</script>

<div class="mt-18 flex w-full flex-col gap-4 px-4 md:w-xl md:px-2">
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
		class="h-14 border-none bg-transparent p-0 text-4xl font-semibold caret-primary outline-none selection:bg-primary selection:text-base-100 placeholder:text-base-content/30 focus:ring-0 focus:outline-none"
		onkeydown={(e) => {
			if (e.key == 'Enter') {
				e.preventDefault();
				e.stopPropagation();
				let editor: HTMLElement | null = document.querySelector('[role="textbox"]');
				editor?.focus();
			}
		}}
	/>
	<main>
		<TipTap />
	</main>
</div>
