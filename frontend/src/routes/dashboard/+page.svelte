<script lang="ts">
	import TagList from '$lib/TagList.svelte';
	import { Bookmark, X } from '@lucide/svelte';

	const sampleData = [
		{
			title: 'Hello World gsdfdsfsd',
			description:
				'This is descriptionsdfdsf sdfsdfsd dfsdffsdf sdf sdfsdf dfs sdfsdf sdf sdf sdfsd fsdfsdfsd',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world',
			tags: ['this', 'is', 'tag-list', 'test', 'more', 'even more', 'even even more']
		},
		{
			title: 'Hello World',
			description: 'This is description of a novel',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world',
			tags: ['this', 'also', 'tag-list']
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world',
			tags: []
		},
		{
			title: '',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description of a novel',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world',
			tags: []
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world',
			tags: []
		}
	];
	const favorites = sampleData.filter((data) => data.favorite);
	const nonfavorites = sampleData.filter((data) => !data.favorite);
</script>

<div class="flex w-full flex-col md:gap-4">
	<div
		class="sticky top-14 z-44 flex flex-col gap-4 border-b border-base-content/10 bg-base-100 p-4 md:w-full md:flex-row md:justify-between"
	>
		<div class="flex flex-row gap-4">
			<button class="btn btn-primary">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="lucide lucide-plus-icon lucide-plus h-5"
					><path d="M5 12h14" /><path d="M12 5v14" /></svg
				>
				New
			</button>
			<div class="join">
				<select class="select join-item w-35">
					<option disabled selected>Sort by</option>
					<option>Modified date</option>
					<option>Creation date</option>
					<option>Name</option>
				</select>
				<select class="select join-item w-20">
					<option>Asc</option>
					<option>Des</option>
				</select>
			</div>
		</div>
		<div class="flex items-center gap-2">
			<a href="/search-help" class="btn btn-circle h-6 w-6 text-base-content/50">?</a>
			<div class="join">
				<label class="input join-item w-full gap-0 md:w-70">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-search-icon lucide-search h-5 text-base-content/50"
						><path d="m21 21-4.34-4.34" /><circle cx="11" cy="11" r="8" /></svg
					>
					<input
						type="search"
						placeholder="filter"
						class="grow focus:ring-0 focus:outline-none focus-visible:outline-none"
					/>
				</label>
				<select class="select join-item w-25">
					<option selected>Name</option>
					<option>Body</option>
					<option>Tags</option>
					<option>Anything</option>
				</select>
			</div>
		</div>
	</div>
	<div class="flex w-full flex-col divide-y md:grid md:grid-cols-3 md:items-start md:gap-4 md:px-4">
		{#each favorites as sd}
			<div class="flex flex-col gap-2 border-base-content/10 p-3 md:rounded-md md:border">
				<div class="flex w-full flex-row items-center justify-between">
					<div class="flex flex-row items-center gap-2">
						<button class="btn h-8 w-auto min-w-0 p-0 btn-link" aria-label="favorite">
							<Bookmark
								fill="currentColor"
								stroke="currentColor"
								class="h-5 text-amber-400 transition duration-100 ease-in-out hover:h-7 hover:rotate-12"
							/>
						</button>
						{#if sd.title}
							<a class="font-semibold" href="/notes/{sd.slug}">
								{sd.title}
							</a>
						{:else}
							<a class="font-semibold text-base-content/40" href="/notes/{sd.slug}"> No title </a>
						{/if}
					</div>
					<button class="btn btn-circle h-7 w-7 p-0 btn-soft btn-error" aria-label="Delete">
						<X class="h-5" />
					</button>
				</div>
				<p class="">
					{sd.description}
				</p>
				<TagList tags={sd.tags} />
				<div class="flex w-full flex-row items-center justify-between">
					<p class="text-sm text-base-content/50">{sd.timestamp}</p>
				</div>
			</div>
		{/each}
	</div>
	<!-- <p class="text-5xl text-error">TODO</p> -->
</div>
