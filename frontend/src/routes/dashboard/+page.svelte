<script lang="ts">
	const sampleData = [
		{
			title: 'Hello World gsdfdsfsd',
			description:
				'This is descriptionsdfdsf sdfsdfsd dfsdffsdf sdf sdfsdf dfs sdfsdf sdf sdf sdfsd fsdfsdfsd',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description of a novel',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description of a novel',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: false,
			slug: 'hello-world'
		},
		{
			title: 'Hello World',
			description: 'This is description',
			timestamp: '20:32 12.06.2025',
			favorite: true,
			slug: 'hello-world'
		}
	];
	const favorites = sampleData.filter((data) => data.favorite);
	const nonfavorites = sampleData.filter((data) => !data.favorite);
</script>

<div class="flex w-full flex-col gap-4 px-4">
	<div class="flex flex-col gap-4 md:flex-row">
		<div class="flex flex-row gap-4">
			<button class="btn">
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
				Create
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
		<label class="input w-full gap-0 md:w-70">
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
	</div>
	<div class="divider">Favorites</div>
	<div class="flex w-full flex-col gap-4 divide-y md:grid md:grid-cols-3 md:items-start">
		{#each favorites as sd}
			<div
				class="flex flex-col items-end gap-4 border-base-content/10 p-4 pt-0 md:justify-between md:rounded-md md:border md:pt-2 md:pb-2"
			>
				<div class="flex w-full flex-col gap-2">
					<div class="flex w-full flex-row items-center justify-between">
						<div class="flex flex-row items-center gap-2">
							<button class="btn w-auto min-w-0 p-0 btn-link" aria-label="favorite">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="24"
									height="24"
									viewBox="0 0 24 24"
									fill="currentColor"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									class="lucide lucide-bookmark-icon lucide-bookmark h-5 text-amber-400"
									><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" /></svg
								>
							</button>
							<a class="font-semibold" href="/notes/{sd.slug}">
								{sd.title}
							</a>
						</div>
						<button
							class="btn h-auto min-h-0 w-auto min-w-0 p-0 btn-link"
							aria-label="Card settings"
						>
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
								class="lucide lucide-ellipsis-icon lucide-ellipsis h-4 text-base-content/50"
								><circle cx="12" cy="12" r="1" /><circle cx="19" cy="12" r="1" /><circle
									cx="5"
									cy="12"
									r="1"
								/></svg
							>
						</button>
					</div>
					<p class="">
						{sd.description}
					</p>
				</div>
				<div class="flex w-full flex-row items-center justify-between">
					<p class="text-sm text-base-content/50">{sd.timestamp}</p>
					<button class="btn text-error no-underline btn-ghost"> Delete </button>
				</div>
			</div>
		{/each}
	</div>
	<div class="divider">Other</div>
	<p class="text-5xl text-error">TODO</p>
</div>
