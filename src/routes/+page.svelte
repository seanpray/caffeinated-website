<script>
	import logo from '$lib/images/logo.png';
	import first_logo from '$lib/images/first.png';
	import Profile from './Profile.svelte';

	import ian_oberbeck from '$lib/headshots/ian_oberbeck.jpeg';
	import jacob_dymkowski from '$lib/headshots/jacob_dymkowski.jpeg';
	import leona_longhurst from '$lib/headshots/leona_longhurst.jpeg';
	import sameeh_pottayil from '$lib/headshots/sameeh_pottayil.jpeg';
	import sean_ray from '$lib/headshots/sean_ray.jpeg';
	import steve_baratian from '$lib/headshots/steve_baratian.jpeg';
	import zach_cloud from '$lib/headshots/zach_cloud.jpeg';

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
		<div class="logo-translate">
			<div class="mobile-steam">
				<div class="grid grid-cols-3 w-[13rem] -translate-y-16">
					<div>
						<div class="dot" />
						<div class="dot delay1" />
						<div class="dot delay2" />
						<div class="dot delay3" />
						<div class="dot delay4" />
						<div class="dot delay5" />
					</div>
					<div class="steam-grid-translate1">
						<div class="dot delay2" />
						<div class="dot delay3" />
						<div class="dot delay4" />
						<div class="dot delay5" />
					</div>
					<div class="steam-grid-translate2">
						<div class="dot delay5" />
					</div>
				</div>
			</div>
            <div class="flex justify-center text-center -translate-y-28">
                <img class="mobile-logo m-1" src={logo} alt="" />
            </div>
		</div>
		<div class="mobile-title">
			<div class="mobile-name text-4xl font-extrabold">caffeinated</div>
			<br />
			<div class="team-number text-xl font-bold ml-1 mt-1">FRC 9293</div>
		</div>
	</div>
	<div class="mt-60" />
    <div class="mx-[1%]">
        <div class="flex flex-wrap justify-center">
            <div class="flex justify-center text-center mr-2">
                <div>
                    <div class="mobile-name uppercase font-extrabold flex ml-2">
                        first
                        {#if !mobile}
                            <img class="ml-16 h-36 m-2 rounded-md" src={first_logo} alt="" />
                        {/if}
                    </div>
                    <div class="flex -translate-y-4">
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
                        <div class="text-boxes m-2 max-w-[45vw] content-center font-bold text-left mt-8">
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

            <div class="flex justify-center mt-4 text-right float-right ml-2">
                <div class="flex flex-wrap justify-center text-right float-right">
                    <div class="text-boxes m-2 max-w-[45vw] content-center font-bold text-right">
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
                                src="https://www.youtube.com/embed/Jd29kzjclV0"
                                title="YouTube video player"
                                frameborder="0"
                                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                allowfullscreen
                            />
                        {:else}
                            <div class="flex flex-center justify-center">
                                <iframe
                                    class="ml-6 rounded-md float-right mt-10"
                                    width="250"
                                    height="141"
                                    src="https://www.youtube.com/embed/Jd29kzjclV0"
                                    title="YouTube video player"
                                    frameborder="0"
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                    allowfullscreen
                                />
                            </div>
                        {/if}
                        <br />
                        <div class="flex first-dots-ending float-right">
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
                        <div class="text-boxes m-2 max-w-[80vw] content-center font-bold text-left mt-8">
                            Caffeinated is a community-based FIRST Robotics Competition team in Marietta, GA. The
                            team was founded on November, 2022 with the mission of not only to be a competitive FRC
                            team, but also to provide a productive, engaging environment for high schoolers in the
                            area to develop advanced skills in engineering and project management. With experienced
                            mentors and coaches around to provide a guiding hand, we are able to organically learn
                            how to come to their own conclusions through their own research and analysis. This
                            allows us a deeper takeaway than simply being given the solution.
                        </div>
                    </div>
                    <div class="flex mt-8">
                        {#if !mobile}
                            <div class="bottom-0 left-4 translate-y-24">
                                <div class="dot2" />
                                <div class="dot2" />
                                <div class="dot2" />
                                <div class="dot2" />
                            </div>
                        {/if}
                        <div class="flex flex-wrap max-w-full team-profile">
                            <img class="mx-4 my-4 h-60 rounded-md" src={ian_oberbeck} alt="" />
                            <img class="mx-4 my-4 h-60 rounded-md" src={jacob_dymkowski} alt="" />
                            <img class="mx-4 my-4 h-60 rounded-md" src={leona_longhurst} alt="" />
                            <img class="mx-4 my-4 h-60 rounded-md" src={sameeh_pottayil} alt="" />
                            <img class="mx-4 my-4 h-60 rounded-md" src={sean_ray} alt="" />
                        </div>
                    </div>
                </div>
            </div>
            {#if !mobile}
                <div class="bottom-0 right-5">
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
		height: min(36rem, 60vh);
		width: min(36rem, 60vh);
	}
	.mobile-title {
        transform: translateY(-22rem);
		margin-top: 30rem;
	}
	.mobile-name {
		font-size: 8vw;
	}
	.mobile-spacing {
		margin-top: 10rem;
	}
	.mobile-steam {
		margin-left: min(10rem, 12vh);
	}
    .first-dots-ending {
        width: 20rem;
    }
	.dot {
		animation-name: wave;
		animation-iteration-count: infinite;
		animation-duration: 5s;
		animation-timing-function: linear;
		height: .8rem;
		width: .8rem;
		border: 1px solid #ffe9d3;
		background: #ffe9d3;
		z-index: 2;
		border-radius: 50%;
		margin: 1.1rem;
	}
    .steam-grid-translate1 {
        margin-top: 3.8rem;
    }
    .steam-grid-translate2 {
        margin-top: 9.5rem;
    }
    .team-profile {}
    .text-boxes {
        font-size: 1.875rem;
        line-height: 2.25rem;
    }
    .logo-translate {
        transform: translateY(-10vh);
    }
	@media (max-width: 1200px) {
        .text-boxes {
            font-size: 1rem;
            line-height: 1.5rem;
        }
        .team-profile {
            display: flex;
            justify-content: center;
        }
        .first-dots-ending {
            width: 90vw;
        }
		.mobile-logo {
			height: 90vw;
			width: 90vw;
            max-width: 500px;
            max-height: 500px;
            margin-top: 1rem;
            margin-left: 10vw;
		}
		.mobile-steam {
			margin-left: 20vw;
            margin-top: 5vh;
		}
        .logo-translate {
            transform: translateY(2vh);
        }
		.mobile-title {
			margin-top: 1rem;
			margin-left: 10vw;
            transform: translateY(max(-25vw, -20vh));
		}
		.mobile-name {
			font-size: 8vw;
		}
		.mobile-spacing {
			margin-top: 1rem;
		}
	}
	.dotcontainer {
		align-items: bottom;
		align-self: bottom;
		bottom: 0;
	}
	.dot2 {
		height: 1rem;
		width: 1rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 2;
		border-radius: 50%;
		margin: 1.7rem;
	}
	.dot3 {
		height: 1rem;
		width: 1rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 2;
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
