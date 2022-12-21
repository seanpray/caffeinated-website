<script lang="ts">
	import rapid_react from '$lib/images/rapidreact.png';
    import start_proto from '$lib/sponsors/startproto.svg';
    import champ from '$lib/sponsors/champ.svg';
    import randisoniker from '$lib/sponsors/randisoniker.png';
</script>

<svelte:head>
	<title>Caffeinated - Sponsors</title>
	<meta name="Caffeinated Robotics" content="Sponsor page" />
</svelte:head>

<section>
	<div class="flex justify-center text-center m-20">
		<div class="text-3xl font-extrabold flex ml-1">
			Many thanks to our sponsors for their generous support!
		</div>
	</div>
	<div class="flex justify-center m-20">
        <div class="flex flex-wrap justify-center m-20 mx-[15%]">
            <img class="h-64 m-8 p-2 bg-white rounded-lg" src={start_proto} alt="" />
            <img class="h-96 m-8 p-2" src={champ} alt="" />
            <img class="h-96 m-8 p-.5" src={randisoniker} alt="" />
        </div>
    </div>
	<div class="flex flex-wrap justify-center text-center m-20">
		<div class="text-2xl font-extrabold flex ml-1">
			Looking to support us? You can contact us at
		</div>
		<div class="text-2xl font-extrabold flex ml-1">
			<a
				class="pl-1.5 rounded-md hover:opacity-90 duration-300"
				href="mailto:caffeinated9293@gmail.com"
				target="_blank"
				rel="noopener noreferrer"
			>
				caffeinated9293@gmail.com
			</a>
		</div>
		<div class="text-2xl font-extrabold flex">
        .
		</div>
	</div>
</section>
