<script lang="ts">
	import { User, Mail, Lock } from '@lucide/svelte';
</script>

<div class="min-w-70 justify-end pt-8">
	<fieldset class="fieldset">
		<legend class="fieldset-legend">Username</legend>
		<label class="validator input gap-0">
			<User class="h-4 opacity-50" />
			<input
				type="text"
				class="grow lowercase focus:ring-0 focus:outline-none focus-visible:outline-none"
				placeholder="JohnDoe123"
				pattern="[A-Za-z0-9]+"
				minlength="3"
				maxlength="16"
				required
			/>
		</label>
		<p class="validator-hint hidden">
			Username must be 3 to 16 lowercase <br /> english letters or digits <br /> (no special symbols
			allowed)
		</p>
	</fieldset>

	<fieldset class="fieldset">
		<legend class="fieldset-legend">Email</legend>
		<label class="validator input gap-0">
			<Mail class="h-4 opacity-50" />
			<input
				type="email"
				class="grow lowercase focus:ring-0 focus:outline-none focus-visible:outline-none"
				placeholder="JohnDoe123@example.com"
				required
			/>
		</label>
		<p class="validator-hint hidden">Input must be a valid email address</p>
	</fieldset>

	<fieldset class="fieldset">
		<legend class="fieldset-legend">Username</legend>
		<label class="validator input gap-0">
			<Lock class="h-4 opacity-50" />
			<input
				type="password"
				class="grow focus:ring-0 focus:outline-none focus-visible:outline-none"
				placeholder="password"
				minlength="8"
				required
			/>
		</label>
		<p class="validator-hint hidden">Password must be at least 8 characters</p>
	</fieldset>

	<button class="btn mt-4 btn-primary">Sign-up</button>
</div>
