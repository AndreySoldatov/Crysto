<div class="min-w-70 justify-end">
	<fieldset class="fieldset">
		<legend class="fieldset-legend">Username or email</legend>
		<label class="validator input gap-0">
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
				class="lucide lucide-user-icon lucide-user h-4 opacity-50"
				><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" /></svg
			>
			<input
				type="text"
				class="grow lowercase focus:ring-0 focus:outline-none focus-visible:outline-none"
				placeholder="JohnDoe123"
				required
			/>
		</label>
	</fieldset>

	<fieldset class="fieldset">
		<legend class="fieldset-legend">Username</legend>
		<label class="validator input gap-0">
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
				class="lucide lucide-lock-icon lucide-lock h-4 opacity-50"
				><rect width="18" height="11" x="3" y="11" rx="2" ry="2" /><path
					d="M7 11V7a5 5 0 0 1 10 0v4"
				/></svg
			>
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
