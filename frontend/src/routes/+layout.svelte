<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	import { Sun, Moon, LayoutDashboard } from '@lucide/svelte';

	let { children } = $props();

	var light_theme = true;
	function switch_theme() {
		if (light_theme) {
			document.documentElement.setAttribute('data-theme', 'amoled');
		} else {
			document.documentElement.setAttribute('data-theme', 'light');
		}
		light_theme = !light_theme;
	}

	import Decypher from '$lib/DecypherRand.svelte';
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="mx-auto flex min-h-screen w-full max-w-4xl flex-col">
	<header
		class="sticky top-0 z-45 grid w-full grid-cols-3 items-center border-b border-base-content/10 bg-base-100 p-2 px-4 print:hidden"
	>
		<a
			href="/"
			class="flex w-30 flex-row items-center gap-2 transition hover:scale-105 hover:rotate-3 active:scale-95"
			><svg
				width="434"
				height="434"
				viewBox="0 0 434 434"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
				class="h-8 w-auto text-primary"
			>
				<path
					d="M409.556 116.847C425.172 146.81 434 180.874 434 217C434 336.846 336.846 434 217 434C162.563 434 112.808 413.955 74.7129 380.844C96.35 330.122 134.777 281.998 180.781 239.211C219.901 202.83 265.118 172.597 312.076 147.312C313.529 146.531 315.122 146.042 316.764 145.876C318.405 145.71 320.063 145.867 321.644 146.342C323.224 146.816 324.695 147.598 325.974 148.641C327.252 149.684 328.312 150.968 329.094 152.421C329.876 153.874 330.364 155.467 330.53 157.108C330.697 158.75 330.539 160.408 330.064 161.988C329.59 163.568 328.809 165.04 327.766 166.318C326.723 167.597 325.438 168.657 323.985 169.438C279.933 193.165 230.839 226.188 188.646 265.425C153.889 297.752 124.301 333.841 105.809 371.938H117.131C171.82 371.938 223.05 357.056 266.969 331.127C275.553 326.052 274.598 313.849 266.014 309.586L247.932 300.583C260.536 299.494 274.891 297.468 287.412 295.458C298.551 293.674 309.664 291.739 320.728 289.545C321.517 289.386 322.244 289.002 322.821 288.439C368.955 243.601 400.477 183.793 409.556 116.847ZM217 0C257.699 0 295.781 11.204 328.33 30.6963C303.353 33.7051 279.332 39.8348 256.667 48.6885C256.148 48.8915 255.675 49.1959 255.275 49.584C254.876 49.9722 254.557 50.4364 254.339 50.9492L254.33 50.9746L254.297 51.0586L254.172 51.3516L253.694 52.4824L251.994 56.6279C250.57 60.1202 248.644 64.9523 246.617 70.3037C243.682 77.9262 241.034 85.6565 238.678 93.4775L225.889 75.6807C224.07 73.1256 221.351 71.355 218.278 70.7256C215.206 70.0962 212.009 70.6548 209.332 72.2891C126.018 123.653 70.1558 215.292 68.8994 320.071C61.7711 331.501 55.3022 343.185 49.6025 355.092C18.6164 317.573 0 269.461 0 217C0 97.1542 97.1542 0 217 0Z"
					fill="currentColor"
				/>
			</svg>
			<Decypher value={'Crysto'} classes="text-xl font-semibold" />
		</a>

		<div class="flex flex-row justify-center gap-x-2">
			<a class="btn btn-ghost" href="/dashboard">
				<LayoutDashboard class="h-5" />
				Dashboard
			</a>
		</div>

		<div class="flex flex-row items-center justify-end gap-4">
			<label class="swap swap-rotate">
				<input
					type="checkbox"
					class="theme-controller hidden"
					value="amoled"
					onclick={switch_theme}
				/>

				<Sun class="swap-on h-5 text-base-content/50" />

				<Moon class="swap-off h-5 text-base-content/50" />
			</label>

			<div class="dropdown dropdown-end">
				<div class="avatar" tabindex="0" role="button">
					<div
						class="h-8 w-auto rounded-full transition hover:scale-105 hover:brightness-70 active:scale-95 active:brightness-50"
					>
						<img
							src="https://www.looper.com/img/gallery/why-aangs-power-in-avatar-the-last-airbender-is-more-terrifying-than-you-think/l-intro-1616420787.jpg"
							alt="Avatar"
						/>
					</div>
				</div>
				<ul
					tabindex="-1"
					class="dropdown-content menu z-50 mt-2 rounded-box border border-base-content/10 bg-base-100 p-2"
				>
					<a class="btn justify-end text-right btn-ghost" href="/account">Account</a>
					<a class="btn justify-end text-right btn-ghost" href="/settings">Settings</a>
					<button
						class="btn justify-end text-right text-error btn-ghost btn-error hover:text-error-content"
						>Logout</button
					>
				</ul>
			</div>
		</div>
	</header>

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
				class="lucide lucide-heart-icon lucide-heart h-4 transition hover:text-error"
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
