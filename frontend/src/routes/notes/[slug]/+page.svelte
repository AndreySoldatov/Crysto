<script lang="ts">
	// const data = {
	// 	title: 'Hello world',
	// 	body: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean eget augue condimentum, sodales libero vitae, finibus lacus. Mauris turpis nisi, condimentum non diam sagittis, vestibulum porttitor est. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. In hac habitasse platea dictumst. In gravida mauris ac ante tempus pretium. Aliquam erat volutpat. Pellentesque vitae facilisis quam. Donec urna ex, venenatis ut interdum at, bibendum eu erat.'
	// };

	// export let value = '';

	// let textarea: HTMLTextAreaElement;

	// function resize() {
	// 	if (!textarea) return;

	// 	// reset height to shrink if text is deleted
	// 	textarea.style.height = 'auto';
	// 	// grow to fit content
	// 	textarea.style.height = textarea.scrollHeight + 'px';
	// }

	// $: resize();

	import TipTap from '$lib/TipTap.svelte';
</script>

<div class="flex w-full flex-col gap-4 px-8 md:w-xl md:px-4">
	<div class="flex w-full flex-col items-center gap-4">
		<p class="pb-4 text-5xl">Hello World</p>
		<!-- <textarea
			bind:this={textarea}
			bind:value
			on:input={resize}
			placeholder="Start typing here..."
			class="w-full resize-none border-none bg-transparent p-0 text-base text-xl leading-relaxed outline-none focus:ring-0 focus:outline-none"
		></textarea> -->
	</div>
	<main>
		<TipTap />
	</main>
</div>
