<script>
    import logo from "$lib/images/logo.png";
    import first_logo from "$lib/images/first.png";
</script>

<svelte:head>
	<title>Caffeinated - Home</title>
	<meta name="Caffeinated Robotics" content="Home page" />
</svelte:head>

<section>
    <div class="bg-[#674736] rounded-3xl m-2 p-4 border-2 border-[#F2D6BE]">
        <div class="flex justify-center text-center">
            <div class="text-5xl">
                Our History
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="flex flex-wrap justify-center text-center content-center">
                <img class="h-80 m-2 rounded-md" src={logo} alt="" />
                <div class="text-2xl m-2 max-w-4xl content-center">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                </div>
            </div>
        </div>
    </div>
    <div class="bg-[#674736] rounded-3xl m-2 p-4 border-2 border-[#F2D6BE]">
        <div class="flex justify-center text-center">
            <div class="text-5xl">
                FIRST Robotics
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="flex flex-wrap justify-center text-center content-center">
                <div class="text-2xl m-2 max-w-4xl content-center">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                </div>
                <img class="h-60 m-2 rounded-md" src={first_logo} alt="" />
            </div>
        </div>
    </div>
    <div class="bg-[#674736] rounded-3xl m-2 p-4 border-2 border-[#F2D6BE]">
        <div class="flex justify-center text-center">
            <div class="text-5xl">
                Team Profile
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="flex flex-wrap justify-center text-center content-center">
                <img class="h-96 m-2 rounded-md" src="http://1.bp.blogspot.com/-m-xzL7vJlnc/UTyHAhrsnUI/AAAAAAAAJP8/V0HDXJBbtFo/s1600/monkey+growl.jpg" alt="" />
                <div class="text-2xl m-2 max-w-4xl content-center">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                </div>
            </div>
        </div>
    </div>
</section>

<style>
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
