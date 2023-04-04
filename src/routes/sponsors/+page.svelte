<script lang="ts">
	import rapid_react from '$lib/images/rapidreact.png';
    import champ from '$lib/sponsors/champ.png';
    import gradienttech from '$lib/sponsors/gradienttech.png';
    import siroco from '$lib/sponsors/siroco.png';
    import birenpatel from '$lib/sponsors/birenpe_logo.png';
    import start_proto from '$lib/sponsors/startproto.svg';
    import homedepot from '$lib/sponsors/homedepot.webp';
    import lowes from '$lib/sponsors/lowes.jpg';

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
            <div class="flex w-4/6 min-w-[10rem] h-auto m-5 p-3 bg-white rounded-lg align-middle justify-center">
                <img src={champ} alt="" />
            </div>
            <div class="flex w-4/6 min-w-[1rem] h-auto m-5 align-middle justify-center">
                <img src={siroco} alt="" />
            </div>
            <div class="flex w-1/6 min-w-[10rem] h-auto m-5 p-3 bg-white rounded-lg align-middle justify-center">
                <img src={birenpatel} alt="" />
            </div>
            <div class="flex w-2/6 min-w-[10rem] h-auto m-5 p-3 rounded-lg align-middle justify-center">
                <img src={gradienttech} alt="" />
            </div>
            <div class="flex w-3/6 min-w-[10rem] h-auto m-5 p-3 bg-white rounded-lg align-middle justify-center">
                <img src={start_proto} alt="" />
            </div>
            <div class="flex w-1/6 min-w-[7rem] h-auto m-5 align-middle justify-center">
                <img class="rounded-sm" src={lowes} alt="" />
            </div>
            <div class="flex w-1/6 min-w-[7rem] h-auto m-5 align-middle justify-center">
                <img class="rounded-sm" src={homedepot} alt="" />
            </div>
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
