<script lang="ts">
	import ls from 'localstorage-slim';
    import { type AuthData, loggedin } from "../../store.ts";
	let auth = false;
	loggedin.subscribe((v) => {
		// store login data here
		auth = v;
	});

	let username = '';
	let password = '';
</script>

<svelte:head>
	<title>Caffeinated - Login</title>
	<meta name="Caffeinated Robotics" content="login page" />
</svelte:head>

<section>
	{#if auth}
		<div class="bg-[#674736] rounded-3xl m-2 p-4 border-2 border-[#F2D6BE]">
			<div class="flex justify-center text-center">
				<div class="text-5xl">Member information</div>
			</div>
			<div class="flex justify-center text-center mt-4">
				<div class="flex flex-wrap justify-center text-center content-center">
					<div class="text-2xl m-2 max-w-4xl content-center">tset</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="bg-[#674736] rounded-3xl m-2 p-4 border-2 border-[#F2D6BE]">
			<div class="flex justify-center text-center">
				<div class="text-5xl">Login</div>
			</div>
			<div class="flex justify-center text-center mt-4">
				<div class="flex flex-wrap justify-center text-center content-center">
					<div class="text-2xl m-2 max-w-4xl content-center">
						<label for="user"> Username </label>
						<br />
						<input id="user" bind:value={username} type="text" />
						<br />
						<label for="pass"> Password </label>
						<br />
						<input id="pass" bind:value={password} type="password" />
						<br />
						<button
							class="border-stone-600 border-2 text-[#674736] bg-[#F2D6BE] rounded-lg p-1 m-2"
							on:click={() => {
								console.log('test');
							}}>Login</button
						>
					</div>
				</div>
			</div>
		</div>
	{/if}
</section>
