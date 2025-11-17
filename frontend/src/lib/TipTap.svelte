<script lang="ts">
	import { fade } from 'svelte/transition';
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { Placeholder } from '@tiptap/extensions';
	import { StarterKit } from '@tiptap/starter-kit';
	import { TaskList, TaskItem } from '@tiptap/extension-list';
	import {
		List,
		ListOrdered,
		Bold,
		Italic,
		Underline,
		Strikethrough,
		Code,
		TextQuote,
		ListChecks,
		Plus,
		Image,
		Highlighter,
		Link as lnk
	} from '@lucide/svelte';
	import { all, createLowlight } from 'lowlight';
	import { CodeBlockLowlight } from '@tiptap/extension-code-block-lowlight';
	import Highlight from '@tiptap/extension-highlight';
	import Typography from '@tiptap/extension-typography';
	import Link from '@tiptap/extension-link';

	const lowlight = createLowlight(all);

	let bubbleMenu = $state();
	let element = $state();
	let editorState: { editor: Editor | null } = $state({ editor: null });

	onMount(() => {
		editorState.editor = new Editor({
			element: element as Element,
			extensions: [
				StarterKit.configure({
					heading: {
						levels: [1, 2, 3]
					}
				}),
				TaskList,
				TaskItem,
				CodeBlockLowlight.configure({
					lowlight,
					tabSize: 4
				}),
				Highlight.configure({
					multicolor: false
				}),
				Placeholder.configure({
					placeholder: 'Write something...'
				}),
				Typography,
				Link.configure({
					openOnClick: true,
					autolink: true,
					defaultProtocol: 'https',
					protocols: ['http', 'https'],
					isAllowedUri: (url, ctx) => {
						try {
							// construct URL
							const parsedUrl = url.includes(':')
								? new URL(url)
								: new URL(`${ctx.defaultProtocol}://${url}`);

							// use default validation
							if (!ctx.defaultValidate(parsedUrl.href)) {
								return false;
							}

							// disallowed protocols
							const disallowedProtocols = ['ftp', 'file', 'mailto'];
							const protocol = parsedUrl.protocol.replace(':', '');

							if (disallowedProtocols.includes(protocol)) {
								return false;
							}

							// only allow protocols specified in ctx.protocols
							const allowedProtocols = ctx.protocols.map((p) =>
								typeof p === 'string' ? p : p.scheme
							);

							if (!allowedProtocols.includes(protocol)) {
								return false;
							}

							// disallowed domains
							const disallowedDomains = ['example-phishing.com', 'malicious-site.net'];
							const domain = parsedUrl.hostname;

							if (disallowedDomains.includes(domain)) {
								return false;
							}

							// all checks have passed
							return true;
						} catch {
							return false;
						}
					},
					shouldAutoLink: (url) => {
						try {
							// construct URL
							const parsedUrl = url.includes(':') ? new URL(url) : new URL(`https://${url}`);

							// only auto-link if the domain is not in the disallowed list
							const disallowedDomains = ['example-no-autolink.com', 'another-no-autolink.com'];
							const domain = parsedUrl.hostname;

							return !disallowedDomains.includes(domain);
						} catch {
							return false;
						}
					}
				})
			],
			content: '',
			onTransaction: ({ editor }) => {
				// Increment the state signal to force a re-render
				editorState = { editor };
			},
			autofocus: true
		});
	});
	onDestroy(() => {
		editorState.editor?.destroy();
	});
</script>

<div style="position: relative" class="app">
	{#if editorState.editor}
		<div
			class="fixed top-14 left-0 z-30 flex w-full flex-row items-center justify-center gap-2 pr-2 md:pr-0"
		>
			<div
				class="no-scrollbar flex w-full flex-row flex-nowrap gap-2 overflow-x-auto rounded-br-md border border-base-content/10 bg-base-100 p-2 shadow-base-content/5 md:w-auto md:rounded-b-md"
			>
				<div class="join">
					<button
						onclick={() => editorState.editor?.chain().focus().toggleBold().run()}
						class={editorState.editor.isActive('bold')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Bold class="h-4 w-4" />
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleItalic().run()}
						class={editorState.editor.isActive('italic')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Italic class="h-4 w-4" />
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleUnderline().run()}
						class={editorState.editor.isActive('underline')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Underline class="h-4 w-4" />
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleStrike().run()}
						class={editorState.editor.isActive('strike')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Strikethrough class="h-4 w-4" />
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleHighlight().run()}
						class={editorState.editor.isActive('highlight')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Highlighter class="h-4 w-4" />
					</button>
				</div>
				<div class="join">
					<button
						onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 1 }).run()}
						class={editorState.editor.isActive('heading', { level: 1 })
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						H1
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 2 }).run()}
						class={editorState.editor.isActive('heading', { level: 2 })
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						H2
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 3 }).run()}
						class={editorState.editor.isActive('heading', { level: 3 })
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						H3
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().setParagraph().run()}
						class={editorState.editor.isActive('paragraph')
							? 'btn join-item px-3 shadow-none btn-primary'
							: 'btn join-item px-3'}
					>
						P
					</button>
				</div>
				<div class="join">
					<button
						onclick={() => editorState.editor?.chain().focus().toggleBulletList().run()}
						class={editorState.editor.isActive('bulletList')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<List class="h-4 w-4" />
					</button>

					<button
						onclick={() => editorState.editor?.chain().focus().toggleOrderedList().run()}
						class={editorState.editor.isActive('orderedList')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<ListOrdered class="h-4 w-4" />
					</button>

					<button
						onclick={() => editorState.editor?.chain().focus().toggleTaskList().run()}
						class={editorState.editor.isActive('taskList')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<ListChecks class="h-4 w-4" />
					</button>
				</div>
				<div class="join">
					<button
						onclick={() =>
							editorState.editor?.state.selection.empty
								? editorState.editor?.chain().focus().toggleCodeBlock().run()
								: editorState.editor?.chain().focus().toggleCode().run()}
						class={editorState.editor.isActive('codeBlock') || editorState.editor.isActive('code')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<Code class="h-4 w-4" />
					</button>
					<button
						onclick={() => editorState.editor?.chain().focus().toggleBlockquote().run()}
						class={editorState.editor.isActive('blockquote')
							? 'btn join-item px-2 shadow-none btn-primary'
							: 'btn join-item px-2'}
					>
						<TextQuote class="h-4 w-4" />
					</button>
				</div>
			</div>

			<div class="dropdown dropdown-end">
				<button tabindex="0" class="btn btn-circle">
					<Plus class="h-5" />
				</button>
				<ul
					tabindex="-1"
					class="dropdown-content menu z-1 mt-4 rounded-md border border-base-content/10 bg-base-100 p-2"
				>
					<li>
						<button class="btn flex flex-row justify-start">
							<Image class="h-4" />
							Image
						</button>
					</li>
				</ul>
			</div>
		</div>
	{/if}
	<div
		bind:this={element}
		class="bg-transparent p-0 leading-relaxed caret-primary outline-none focus:ring-0 focus:outline-none"
	></div>
</div>
