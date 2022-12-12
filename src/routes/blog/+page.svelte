<script lang="ts">
	import { ismobile } from '../../store.ts';
	import logo from '$lib/images/logo.png';
	import Post from '../../BlogPost.svelte';
	// Plan for blog image:
	// schema for post
	const post = [
		{
			title: 'test',
			date: '12/12/12',
			text: 'Hello world',
			images: ['url1', 'url2']
		}
	];
    let menuOpen = false;
	let mobile = false;
    ismobile.subscribe((d) => {
        mobile = d;
    });
</script>

<svelte:head>
	<title>Caffeinated - Blog</title>
	<meta name="Caffeinated Robotics" content="Blog page" />
</svelte:head>

<img class="absolute h-48" src={logo} alt="" />
<section class="flex mx-[5%] mt-20">
<div class="navbar">
    <div class="container nav-container mb-10">
        <input
            bind:checked={menuOpen}
            class="checkbox"
            type="checkbox"
            name=""
            id=""
        />
        <div class="hamburger-lines">
            <span class="line line1" />
            <span class="line line2" />
            <span class="line line3" />
        </div>
        <div class="menu-items font-bold rounded-md">
            <a class="p-2 hover:underline" on:click={() => menuOpen = false} href="/">Home</a>
            <br />
            <a class="p-2" on:click={() => menuOpen = false}>
                <div class="underline">
                    Robot
                </div>
                <div>
                    <br />
                    <a href="/robots/powerplay">Power Play</a>
                    <br />
                    <a href="/robots/freightfrenzy">Freight Frezy</a>
                    <br />
                </div>
            </a>
            <br />
            <a class="p-2" on:click={() => menuOpen = false}>
                <div class="underline">
                    About
                </div>
                <div>
                    <br />
                    <a href="/about/first">FIRST</a>
                    <br />
                    <a href="/about/team">Team</a>
                    <br />
                    <a href="/about/achievements">Achievements</a>
                    <br />
                </div>
            </a>
            <br />
            <a class="p-2" on:click={() => menuOpen = false}>
                <div class="underline">
                    Sponsors
                </div>
                <div>
                    <br />
                    <a href="/sponsors/donate">Donate</a>
                    <br />
                    <a href="/sponsors">Sponsors</a>
                    <br />
                </div>
            </a>
        </div>
    </div>
</div>
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
		<Post title={'title temp'} date={'unix timestamp goes here for date'} text={'hello'} />
	</div>
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
</section>

<style>
.nav-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.navbar .menu-items {
    z-index: 10;
    display: flex;
}
.navbar .nav-container li {
    list-style: none;
}
.navbar .nav-container a {
    text-decoration: none;
    color: #fff;
    font-weight: 500;
    font-size: 1.2rem;
    padding: 0.7rem;
}
.navbar .nav-container a:hover {
    font-weight: bolder;
}
.nav-container {
    display: block;
}
.nav-container .checkbox {
    position: absolute;
    right: 20px;
    display: block;
    height: 32px;
    width: 32px;
    top: 20px;
    z-index: 500;
    opacity: 0;
    cursor: pointer;
}
.nav-container .hamburger-lines {
    display: block;
    height: 26px;
    width: 32px;
    position: absolute;
    top: 17px;
    right: 20px;
    z-index: 2;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
}
.nav-container .hamburger-lines .line {
    display: block;
    height: 4px;
    width: 100%;
    border-radius: 10px;
    background: #fff;
}
.nav-container .hamburger-lines .line1 {
    transform-origin: 0% 0%;
    transition: transform 0.4s ease-in-out;
}
.nav-container .hamburger-lines .line2 {
    transition: transform 0.2s ease-in-out;
}
.nav-container .hamburger-lines .line3 {
    transform-origin: 0% 100%;
    transition: transform 0.4s ease-in-out;
}
.navbar .menu-items {
    box-shadow: inset 00 2000px rgba(0, 0, 0, 0.8);
    margin-top: 120px;
    height: fit-content;
    width: 50%;
    transform: translate(-150%);
    display: flex;
    flex-direction: column;
    transition: transform 0.3s ease-in-out;
    text-align: center;
    position: fixed;
    margin-right: 0px;
}
.navbar .menu-items li {
    margin-bottom: 1.2rem;
    font-size: 1.5rem;
    font-weight: 500;
}
.logo {
    position: absolute;
    top: 5px;
    right: 15px;
    font-size: 1.2rem;
    color: #fff;
}
.nav-container input[type="checkbox"]:checked ~ .menu-items {
    transform: translateX(0);
}
.nav-container input[type="checkbox"]:checked ~ .hamburger-lines .line1 {
    transform: rotate(45deg);
}
.nav-container input[type="checkbox"]:checked ~ .hamburger-lines .line2 {
    transform: scaleY(0);
}
.nav-container input[type="checkbox"]:checked ~ .hamburger-lines .line3 {
    transform: rotate(-45deg);
}
.nav-container input[type="checkbox"]:checked ~ .logo {
    display: none;
}
</style>
