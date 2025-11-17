<script lang="ts">
	import { onMount } from 'svelte';

	function randomString(length = 16) {
		const chars =
			'abcdefghijklmnopqrstuvwxyz' +
			'ABCDEFGHIJKLMNOPQRSTUVWXYZ' +
			'0123456789' +
			'!@#$%^&*()_+-=[]{}|;:,.<>/?';

		const array = new Uint32Array(length);
		crypto.getRandomValues(array);

		return Array.from(array, (x) => chars[x % chars.length]).join('');
	}

	export let classes = '';
	export let value: string = 'Hello World';
	export let duration = 50;

	var currentValue = value;

	let onEnter = () => {
		if (currentValue != value) return;
		currentValue = randomString(value.length);
	};

	onMount(() => {
		const animation = setInterval(() => {
			if (currentValue != value) {
				var index = Math.floor(Math.random() * value.length);
				while (currentValue[index] == value[index]) {
					index = Math.floor(Math.random() * value.length);
				}
				currentValue =
					currentValue.slice(0, index) + value[index] + currentValue.slice(index + 1, value.length);
			}
		}, duration);

		return () => clearInterval(animation);
	});
</script>

<p on:mouseenter={onEnter} class={classes}>{currentValue}</p>
