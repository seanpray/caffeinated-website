<script>
	import logo from '$lib/images/logo.png';
	import first_logo from '$lib/images/first.png';
	import Profile from './Profile.svelte';

    import ian_oberbeck from "$lib/headshots/ian_oberbeck.jpeg";
    import jacob_dymkowski from "$lib/headshots/jacob_dymkowski.jpeg";
    import leona_longhurst from "$lib/headshots/leona_longhurst.jpeg";
    import sameeh_pottayil from "$lib/headshots/sameeh_pottayil.jpeg";
    import sean_ray from "$lib/headshots/sean_ray.jpeg";
    import steve_baratian from "$lib/headshots/steve_baratian.jpeg";
    import zach_cloud from "$lib/headshots/zach_cloud.jpeg";

	import { getContext } from 'svelte';
	import { browser } from '$app/environment';
	import { ismobile } from '../store.ts';
	let mobile = false;
	if (browser) {
		ismobile.subscribe((d) => {
			mobile = d;
		});
	}
</script>

<svelte:head>
	<title>Caffeinated - About</title>
	<meta name="Caffeinated Robotics" content="Home page" />
</svelte:head>

<section>
	<div class="flex flex-wrap mobile-spacing">
		<div>
			{#if !mobile}
				<div class="box-top-left" />
				<div class="box-top-left2" />
				<div class="box-top-right" />
				<div class="box-top-right2" />
				<div class="absolute bottom-6 left-0">
					<div class="dot2" />
					<div class="dot2" />
				</div>
				<div class="absolute bottom-12 right-0">
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
					<div class="dot2" />
				</div>
			{/if}
		</div>
		<div>
			<div class="mobile-steam">
				<div class="grid grid-cols-3 gap-0.5 w-[13rem]">
					<div>
						<div class="dot" />
						<div class="dot delay1" />
						<div class="dot delay2" />
						<div class="dot delay3" />
						<div class="dot delay4" />
						<div class="dot delay5" />
					</div>
					<div class="mt-[6rem]">
						<div class="dot delay2" />
						<div class="dot delay3" />
						<div class="dot delay4" />
						<div class="dot delay5" />
					</div>
					<div class="mt-[15rem]">
						<div class="dot delay5" />
					</div>
				</div>
			</div>
			<img class="mobile-logo m-2" src={logo} alt="" />
		</div>
		<div class="mobile-title">
			<div class="mobile-name font-extrabold">caffeinated</div>
			<br />
			<div class="team-number text-4xl font-bold ml-1">FRC 9293</div>
		</div>
	</div>
	<div class="mt-60" />
	<!-- caffeinated9293@gmail.com -->
	<div class="flex flex-wrap">
		<div class="flex justify-center text-center">
			<div>
				<div class="mobile-name uppercase font-extrabold flex ml-1">
					first
                    {#if !mobile}
                        <img class="ml-16 h-48 m-2 rounded-md" src={first_logo} alt="" />
                    {/if}
				</div>
				<div class="flex -translate-y-8">
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
					<div class="dot3" />
				</div>
				<div>
					<div class="text-4xl m-2 max-w-4xl content-center font-bold text-left mt-8">
						In 1989, Dean Kamen started the nonprofit organization FIRST, which stands for "For the
						Inspiration and Recognition of Science and Technology". The FIRST Robotics Competition
						is a high-school robotics competition that combines gracious professionalism with
						technology to create an environment that encourages a better appreciation for STEM.
						Professional mentors and college students collaborate with teams to educate students in
						essential engineering concepts and professional skills while also assisting the team in
						its success.
					</div>
				</div>
			</div>
		</div>

		<div class="flex justify-center text-center mt-4">
			<div class="flex flex-wrap justify-center text-center content-center">
				<div class="text-3xl m-2 max-w-4xl content-center font-bold text-right">
					"FIRST creates programs where its participants are limited only by the bounds of their
					imagination and gives them a forum to begin to examine the world’s problems by exploring
					science, technology, engineering and math.
					<br />
					- Congressman Jim Langevin (RI)
                    {#if !mobile}
                        <iframe
                            class="ml-6 rounded-md float-right mt-10"
                            width="560"
                            height="315"
                            src="https://www.youtube.com/embed/oVhzG42vOoY"
                            title="YouTube video player"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen
                        />
                    {:else}
                        <iframe
                            class="ml-6 rounded-md float-right mt-10"
                            width="267"
                            height="150"
                            src="https://www.youtube.com/embed/oVhzG42vOoY"
                            title="YouTube video player"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen
                        />
                    {/if}
					<br />
					<div class="flex w-[20rem] float-right">
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
						<div class="dot3" />
					</div>
				</div>
			</div>
		</div>
	</div>
    <div class="mt-80" />
	<div class="flex flex-wrap">
		<div class="flex justify-center text-center">
			<div>
				<div class="mobile-name font-extrabold flex ml-1">the team</div>
				<div>
					<div class="text-4xl m-2 max-w-[110rem] content-center font-bold text-left mt-8">
						Caffeinated is a community-based FIRST Robotics Competition team in Marietta, GA. The
						team was founded on November, 2022 with the mission of not only to be a competitive FRC
						team, but also to provide a productive, engaging environment for high schoolers in the
						area to develop advanced skills in engineering and project management. With experienced
						mentors and coaches around to provide a guiding hand, we are able to organically learn
						how to come to their own conclusions through their own research and analysis. This
						allows us a deeper takeaway than simply being given the solution.
					</div>
				</div>
                <div class="flex -translate-x-16 mt-8">
                    <div class="bottom-0 left-0 translate-y-24">
                        <div class="dot2" />
                        <div class="dot2" />
                        <div class="dot2" />
                        <div class="dot2" />
                    </div>
                    <div class="flex flex-wrap max-w-full">
                        <img class="mx-4 h-64" src={ian_oberbeck} alt="" />
                        <img class="mx-4 h-64" src={jacob_dymkowski} alt="" />
                        <img class="mx-4 h-64" src={leona_longhurst} alt="" />
                        <img class="mx-4 h-64" src={sameeh_pottayil} alt="" />
                        <img class="mx-4 h-64" src={sean_ray} alt="" />
                    </div>
                </div>
			</div>
		</div>
            {#if !mobile}
                <div class="bottom-0 right-0">
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                </div>
            {/if}
	</div>
</section>

<style>
	.box-top-left {
		background: #fcebda;
		top: 0;
		left: 0;
		position: absolute;
		width: 10vw;
		height: 10vw;
	}
	.box-top-left2 {
		background: #4c2700;
		top: 0;
		left: 0;
		position: absolute;
		width: 6vw;
		height: 70vh;
		margin-top: 8vw;
		border: 2px solid #fcebda;
	}
	.box-top-right {
		background: #4c2700;
		top: 0;
		right: 0;
		position: absolute;
		width: 10vw;
		height: 40vh;
		border-bottom: 2px solid #fcebda;
		border-left: 2px solid #fcebda;
	}
	.box-top-right2 {
		background: #fcebda;
		top: 0;
		right: 0;
		margin-top: 25vh;
		position: absolute;
		width: 5vw;
		height: 30vh;
	}
	.mobile-logo {
		height: 36rem;
		width: 36rem;
	}
	.mobile-title {
		margin-top: 30rem;
	}
	.mobile-name {
		font-size: 8rem;
	}
	.mobile-spacing {
		margin-top: 10rem;
	}
	.mobile-steam {
		margin-left: 10rem;
	}
	@media (max-width: 1000px) {
		.mobile-title {
			margin-top: 0;
			margin-left: 2.5rem;
		}
		.mobile-logo {
			height: 25rem;
			width: 25rem;
		}
		.mobile-name {
			font-size: 17vw;
		}
		.mobile-spacing {
			margin-top: 1rem;
		}
		.mobile-steam {
			margin-left: 6rem;
		}
	}
	.dotcontainer {
		align-items: bottom;
		align-self: bottom;
		bottom: 0;
	}
	.dot {
		animation-name: wave;
		animation-iteration-count: infinite;
		animation-duration: 5s;
		animation-timing-function: linear;
		height: 1.3rem;
		width: 1.3rem;
		border: 1px solid #ffe9d3;
		background: #ffe9d3;
		z-index: 100;
		border-radius: 50%;
		margin: 1.7rem;
	}
	.dot2 {
		height: 1.3rem;
		width: 1.3rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 100;
		border-radius: 50%;
		margin: 1.7rem;
	}
	.dot3 {
		height: 1.3rem;
		width: 1.3rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 100;
		border-radius: 50%;
		margin: 0.6rem;
	}
	.delay1 {
		animation-delay: 0.5s;
	}
	.delay2 {
		animation-delay: 1s;
	}
	.delay3 {
		animation-delay: 1.5s;
	}
	.delay4 {
		animation-delay: 2s;
	}
	.delay5 {
		animation-delay: 2.5s;
	}
	.delay6 {
		animation-delay: 3s;
	}
	.delay7 {
		animation-delay: 3.5s;
	}
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
