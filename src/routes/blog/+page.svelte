<script lang="ts">
    import logo from "$lib/images/logo.png";
    import Post from "../../BlogPost.svelte";
</script>

<svelte:head>
	<title>Caffeinated - Blog</title>
	<meta name="Caffeinated Robotics" content="Blog page" />
</svelte:head>

<img class="absolute h-48" src={logo} alt="" />
<section class="flex mx-[5%] mt-20">
    <div class="bg-[#bc8f62] mt-28 text-white w-2/12 h-[70vh] text-5xl font-extrabold">
        <div class="flex justify-center text-center mt-10">
            2022
            <br />
            12 6
            <br />
            Tuesday
        </div>
    </div>
    <div>
        <Post title={"title temp"} date={"unix timestamp goes here for date"} text={"hello"} />
    </div>
    <div class="absolute right-20 bg-[#4c2700] w-2/12 text-[#bc8f62] h-[85vh]">
        <div class="ml-10 mt-4">
            <div class="text-6xl font-extrabold">
                blog posts
            </div>
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
</section>
