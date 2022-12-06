<script lang="ts">
    import rapid_react from "$lib/images/rapidreact.png";
    import Stl from "../../STL.svelte";
</script>

<svelte:head>
	<title>Caffeinated - Robot</title>
	<meta name="Caffeinated Robotics" content="Robot page" />
</svelte:head>

<section class="mt-20">
    <div class="flex justify-center text-center mt-20">
        <div class="text-5xl font-extrabold flex ml-1">robot</div>
    </div>
    <div class="text-4xl font-extrabold flex ml-1">
        will make this a tab system soon for diff seasons
    </div>
    <div class="flex justify-center text-center mt-4">
        <Stl />
    </div>
    <div class="flex justify-center text-center">
        <div class="flex flex-wrap justify-center text-center content-center">
            <img class="h-80 m-2 rounded-md" src={rapid_react} alt="" />
            <div class="text-2xl m-2 max-w-4xl content-center">
                Rapid React is the game for the 2022 FIRST Robotics Competition. The game is themed around transportation and involves two alliances consisting of three teams each competing to perform various tasks, including shooting balls known as Cargo into high and low hubs, and hanging on a series of rungs at the end of the match.
            </div>
        </div>
    </div>
    <div class="flex justify-center text-center">
        <iframe class="min-w-[50vw] min-h-[28vw] rounded-xl" src="https://www.youtube.com/embed/LgniEjI9cCM" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen />
    </div>
</section>
