<script lang="ts">
	import Decypher from '$lib/DecypherRand.svelte';
</script>

<svelte:head>
	<title>Crysto</title>
</svelte:head>

<div class="flex w-1/2 min-w-80 flex-col gap-6 p-8">
	<div class="flex flex-col items-center">
		<Decypher value={'Crysto'} classes="text-3xl font-semibold hover:scale-110 transition" />
	</div>
	<p>
		Note taking app for security paranoids. We store only your login in plain text form, everything
		else is
		<a
			class="text-primary underline"
			target="_blank"
			href="https://en.wikipedia.org/wiki/End-to-end_encryption">e2ee</a
		> by default. We don't have the keys from your skeleton closet.
	</p>

	<div class="flex flex-row gap-4">
		<a class="btn btn-primary" href="/signup">Sign-up</a>
		<a class="btn border-base-content/10 btn-outline" href="/login">Login</a>
	</div>
</div>
