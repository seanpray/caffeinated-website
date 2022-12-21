<script lang="ts">
	import { ismobile } from '../../store.ts';
	import logo from '$lib/images/logo.png';
	import Post from '../../BlogPost.svelte';
	import { onMount } from 'svelte';
	// Plan for blog image:
	// schema for post
	let posts = [];
	let mobile = false;
	ismobile.subscribe((d) => {
		mobile = d;
	});
	onMount(async () => {
    console.log(import.meta.env.VITE_BACKEND_URL)
		let req = await fetch(import.meta.env.VITE_BACKEND_URL + "/posts");
		if (req.ok) {
			posts = await req.json();
		}
		console.log(posts);
	});

	const prefixZero = (num: number) => {
		return num < 10 ? '0' + num : num;
	};

	const formatDate = (unix_timestamp: number) => {
		const postdate = new Date(unix_timestamp * 1000);
		let day = prefixZero(postdate.getDate());
		let month = prefixZero(postdate.getMonth() + 1);
		let weekday = 'Sunday';
		switch (postdate.getDay()) {
			case 1:
				weekday = 'Monday';
				break;
			case 2:
				weekday = 'Tuesday';
				break;
			case 3:
				weekday = 'Wenesday';
				break;
			case 4:
				weekday = 'Thursday';
				break;
			case 5:
				weekday = 'Friday';
				break;
			case 6:
				weekday = 'Saturday';
				break;
		}
		return `
            ${postdate.getFullYear()}
            <br />
            ${month}/${day}
            <br />
            ${weekday}
        `;
	};
</script>

<svelte:head>
	<title>Caffeinated - Blog</title>
	<meta name="Caffeinated Robotics" content="Blog page" />
</svelte:head>

{#if !mobile}
	<img class="absolute h-48 mt-4 ml-[8%]" src={logo} alt="" />
{:else}
	<div class="bg-[#4c2700] mt-36 mx-[10%] text-[#bc8f62] p-7">
		<div class="ml-10 mt-4">
			<div class="text-6xl font-extrabold">blog posts</div>
            <div class="text-3xl font-extrabold">
                {#each posts as p}
                    <a href={'#' + p.title}>
                        {p.title}
                    </a>
                    <br />
                {/each}
            </div>
		</div>
	</div>
{/if}
<section class="flex mx-[5%] mt-20">
	{#if !mobile}
		<div class="bg-[#bc8f62] mt-40 text-white w-2/12 h-[70vh] text-5xl font-extrabold">
			<div class="flex justify-center text-center mt-10">
				{#each posts as p}
					{@html formatDate(p.created_date)}
                    <br />
                    <br />
				{/each}
			</div>
		</div>
	{/if}
	<div>
		{#each posts as p}
			<Post data={p} />
		{/each}
	</div>
	{#if !mobile}
		<div class="absolute right-20 bg-[#4c2700] w-2/12 text-[#bc8f62] h-[85vh]">
			<div class="ml-10 mt-10">
				<div class="text-6xl font-extrabold">blog posts</div>
				<div class="text-3xl font-extrabold">
					{#each posts as p}
						<a href={'#' + p.title}>
							{p.title}
						</a>
                        <br />
					{/each}
				</div>
			</div>
		</div>
	{/if}
</section>
