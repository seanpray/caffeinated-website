<script>
    import logo from "$lib/images/logo.png";
    import first_logo from "$lib/images/first.png";
    import Profile from "./Profile.svelte";

    // place holder monkey
    const m = "http://1.bp.blogspot.com/-m-xzL7vJlnc/UTyHAhrsnUI/AAAAAAAAJP8/V0HDXJBbtFo/s1600/monkey+growl.jpg";

    const students = [[m, "Ian Oberbeck"], [m, "Sean Ray"], [m, "Jacob Dymkowski"], [m, "Sameeh Pottayil"], [m, "Leona Longhurst"]];
    const mentors = [[m, "Kent Oberbeck"], [m, "Steve Baratian"], [m, "Zach Cloud"], [m, "Jacob Oberbeck"]];
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
                    Caffeinated is a community FIRST Robotics Competition team based in Marietta, GA. The team
                    was founded in November, 2022 with the mission of not only being a competitive FRC team, but
                    also providing a productive environment for highschoolers to learn engineering and project
                    management. We believe in the “Student-Driven” model; our coaches provide the framework
                    and guidance by which students organically learn how to come to their own conclusions through
                    their own research and analysis, providing a deeper level of understanding than simply being
                    given the solution and explanation.
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
                    The FIRST Robotics Competition is a high-school robotics program run by For Inspiration and
                    Recognition of Science and Technology, or FIRST, an international not-for-profit organization
                    founded in 1989. FRC teams design, build, and program industrial sized robots during an
                    intense 6 week long program that culminates in a series of competitions that test it’s abilities.
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
                    Caffeinated is located in the Peachtree District, encompassing Georgia and South Carolina, and
                    will be competing in the Gwinnett and Dalton Qualifiers as well as, hopefully, District Champs
                    and FIRST Champs.The Peachtree District is an amazing community that truly embodies
                    FIRST’s “Coopertition,” the practice of fiercely competing whilst still fostering a cooperative
                    environment that prompts mutual gain.
                    <br />
                    <br />
                    Caffeinated is located in the Peachtree District, encompassing Georgia and South Carolina, and
                    will be competing in the Gwinnett and Dalton Qualifiers as well as, hopefully, District Champs
                    and FIRST Champs.The Peachtree District is an amazing community that truly embodies
                    FIRST’s “Coopertition,” the practice of fiercely competing whilst still fostering a cooperative
                    environment that prompts mutual gain.
                </div>
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
            <div class="text-4xl">
                Students
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="flex flex-wrap justify-center text-center content-center">
                <div class="text-2xl m-2 max-w-4xl content-center">
                    <div class="flex justify-center flex-wrap">
                        {#each students as s}
                            <Profile image={s[0]} name={s[1]} />
                        {/each}
                    </div>
                </div>
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="text-4xl">
                Mentors
            </div>
        </div>
        <div class="flex justify-center text-center">
            <div class="flex flex-wrap justify-center text-center content-center">
                <div class="text-2xl m-2 max-w-4xl content-center">
                    <div class="flex justify-center flex-wrap">
                        {#each mentors as s}
                            <Profile image={s[0]} name={s[1]} />
                        {/each}
                    </div>
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
