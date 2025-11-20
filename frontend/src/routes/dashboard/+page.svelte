<script lang="ts">
	import TagList from '$lib/TagList.svelte';
	import { Bookmark, X, Plus, Search } from '@lucide/svelte';

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

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<div class="flex w-full flex-col md:gap-4">
	<div
		class="sticky top-14 z-44 flex flex-col gap-4 border-b border-base-content/10 bg-base-100 p-4 md:w-full md:flex-row md:justify-between"
	>
		<div class="flex flex-row justify-between gap-4">
			<button class="btn pl-3 btn-primary">
				<Plus class="h-5" />
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
			<div class="join w-full">
				<label class="input join-item gap-0 md:w-70">
					<Search class="h-5 text-base-content/50" />
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
								class="h-5 text-amber-400 transition duration-100 ease-in-out hover:scale-110 hover:rotate-12"
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
</div>
