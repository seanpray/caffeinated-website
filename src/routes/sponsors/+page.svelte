<script lang="ts">
	import rapid_react from '$lib/images/rapidreact.png';
    import start_proto from '$lib/sponsors/startproto.svg';
    import champ from '$lib/sponsors/champ.png';
    import randisoniker from '$lib/sponsors/randisoniker.png';

    import gra from '$lib/gra.jpg'
</script>

<svelte:head>
	<title>Caffeinated - Sponsors</title>
	<meta name="Caffeinated Robotics" content="Sponsor page" />
</svelte:head>

<section>
	<div class="flex justify-center text-center m-1">
		<div class="text-3xl font-extrabold flex mt-4 ml-1">
			Many thanks to our sponsors for their generous support!
		</div>
	</div>
	<div class="flex justify-center">
        <div class="flex flex-wrap justify-center mx-[15%]">
            <div class="flex w-2/6 min-w-[18rem] h-auto m-8 p-3 bg-white rounded-lg align-middle justify-center">
                <img src={start_proto} alt="" />
            </div>
            <div class="flex w-2/6 min-w-[18rem] h-auto m-8 p-3 bg-white rounded-lg align-middle justify-center">
                <img src={champ} alt="" />
            </div>
            <!-- <div class="flex w-2/6 min-w-[18rem] h-auto m-8 p-3 align-middle justify-center"> -->
            <!--     <img src={randisoniker} alt="" /> -->
            <!-- </div> -->
        </div>
    </div>
	<div class="flex flex-wrap text-xl justify-center text-center mt-5">
        Caffeinated FRC 9293 is a member of the Georgia Robotics Alliance.
    </div>
	<div class="flex flex-wrap justify-center text-center">
        <img class="max-w-[70vw] rounded-md" src={gra} alt="" />
    </div>
	<div class="flex flex-wrap justify-center text-center m-20">
		<div class="text-md font-extrabold flex ml-1">
			Looking to support us? You can contact us at
		</div>
		<div class="text-md font-extrabold flex ml-1">
			<a
				class="pl-1.5 rounded-md hover:opacity-90 duration-300"
				href="mailto:caffeinated9293@gmail.com"
				target="_blank"
				rel="noopener noreferrer"
			>
				caffeinated9293@gmail.com
			</a>
		</div>
	</div>
</section>
