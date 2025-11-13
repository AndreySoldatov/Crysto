<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { StarterKit } from '@tiptap/starter-kit';

	let bubbleMenu = $state();
	let element = $state();
	let editorState: { editor: Editor | null } = $state({ editor: null });

	onMount(() => {
		editorState.editor = new Editor({
			element: element as Element,
			extensions: [StarterKit],
			content: `
        <h1>Hello Svelte! 🌍️ </h1>
        <p>This editor is running in Svelte.</p>
        <p>Select some text to see the bubble menu popping up.</p>
      `,
			onTransaction: ({ editor }) => {
				// Increment the state signal to force a re-render
				editorState = { editor };
			}
		});
	});
	onDestroy(() => {
		editorState.editor?.destroy();
	});
</script>

<div style="position: relative" class="app">
	{#if editorState.editor}
		<div class="fixed-menu">
			<button
				onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 1 }).run()}
				class:active={editorState.editor.isActive('heading', { level: 1 })}
			>
				H1
			</button>
			<button
				onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 2 }).run()}
				class:active={editorState.editor.isActive('heading', { level: 2 })}
			>
				H2
			</button>
			<button
				onclick={() => editorState.editor?.chain().focus().setParagraph().run()}
				class:active={editorState.editor.isActive('paragraph')}
			>
				P
			</button>
		</div>
	{/if}

	<div bind:this={element}></div>
</div>

<style>
	button.active {
		background: black;
		color: white;
	}
</style>
