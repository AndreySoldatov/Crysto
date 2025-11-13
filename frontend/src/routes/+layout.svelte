<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="mx-auto flex min-h-dvh min-h-screen w-full max-w-4xl flex-col">
	<header class="flex w-full items-center justify-between p-4 px-6 print:hidden">
		<!-- <div class="flex flex-row gap-x-2">
			<a class="text-2xl font-semibold" href="/">Crysto</a>
		</div> -->
		<a href="/" class="text-xl font-semibold">Crysto</a>
		<div class="flex flex-row gap-x-2">
			<a class="btn border-base-content/10 btn-outline" href="/dashboard">
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
					class="lucide lucide-layout-dashboard-icon lucide-layout-dashboard h-5"
					><rect width="7" height="9" x="3" y="3" rx="1" /><rect
						width="7"
						height="5"
						x="14"
						y="3"
						rx="1"
					/><rect width="7" height="9" x="14" y="12" rx="1" /><rect
						width="7"
						height="5"
						x="3"
						y="16"
						rx="1"
					/></svg
				>
				Dashboard
			</a>
		</div>

		<div class="dropdown dropdown-end">
			<div class="avatar" tabindex="0" role="button">
				<div class="w-10 rounded-full hover:brightness-70">
					<img
						src="https://www.looper.com/img/gallery/why-aangs-power-in-avatar-the-last-airbender-is-more-terrifying-than-you-think/l-intro-1616420787.jpg"
						alt="Avatar"
					/>
				</div>
			</div>
			<ul
				tabindex="-1"
				class="dropdown-content menu z-1 rounded-box border border-base-content/10 bg-base-100 p-2"
			>
				<a class="btn justify-end text-right btn-ghost" href="/account">Account</a>
				<a class="btn justify-end text-right btn-ghost" href="/settings">Settings</a>
				<button
					class="btn justify-end text-right text-error btn-ghost btn-error hover:text-error-content"
					>Logout</button
				>
			</ul>
		</div>
	</header>

	<div class="mx-4 mb-8 border-b border-base-content/10 print:hidden"></div>

	<div class="flex w-full flex-1 flex-col items-center">
		{@render children()}
	</div>

	<footer class="flex flex-col items-center gap-2 p-4 text-base-content/50 print:hidden">
		<div class="flex flex-row items-center">
			<p>Made with</p>
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
				class="lucide lucide-heart-icon lucide-heart h-4"
				><path
					d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"
				/></svg
			>
		</div>
		<p>
			Copyright <a
				href="https://github.com/AndreySoldatov"
				class="text-primary underline"
				target="_blank">Soldatov Andrey</a
			>
		</p>
	</footer>
</div>
