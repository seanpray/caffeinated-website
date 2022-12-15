<script lang="ts">
	import { ismobile } from '../../store.ts';
	import logo from '$lib/images/logo.png';
	import Post from '../../BlogPost.svelte';
    import { loadPostMeta } from "./+page.ts";
	// Plan for blog image:
	// schema for post
	const post = [
		{
			title: 'test',
			date: '12/12/12',
			text: 'Hello world',
		}
	];
	let mobile = false;
    ismobile.subscribe((d) => {
        mobile = d;
    });
</script>

<svelte:head>
	<title>Caffeinated - Blog</title>
	<meta name="Caffeinated Robotics" content="Blog page" />
</svelte:head>

{#if !mobile}
    <img class="absolute h-48 mt-10" src={logo} alt="" />
{:else}
    <div class="bg-[#4c2700] mt-28 mx-[10%] text-[#bc8f62] p-7">
        <div class="ml-10 mt-4">
            <div class="text-6xl font-extrabold">blog posts</div>
            <div class="text-3xl font-extrabold">
                Hello World!
                <br />
                blog posts
                <br />
                blog posts
                <br />
                blog posts
                <br />
            </div>
        </div>
    </div>
{/if}
<section class="flex mx-[5%] mt-20">
    {#if !mobile}
        <div class="bg-[#bc8f62] mt-36 text-white w-2/12 h-[70vh] text-5xl font-extrabold">
            <div class="flex justify-center text-center mt-10">
                2022
                <br />
                12 6
                <br />
                Tuesday
            </div>
        </div>
    {/if}
	<div>
        {#each post as p}
            <Post title={p.title} date={p.date} text={p.text} />
        {/each}
	</div>
    {#if !mobile}
        <div class="absolute right-20 bg-[#4c2700] w-2/12 text-[#bc8f62] h-[85vh]">
            <div class="ml-10 mt-4">
                <div class="text-6xl font-extrabold">blog posts</div>
                <div class="text-3xl font-extrabold">
                    Hello World!
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                    <br />
                    blog posts
                </div>
            </div>
        </div>
    {/if}
</section>
