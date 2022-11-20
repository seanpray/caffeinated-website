<script>
	import Header from './Header.svelte';
	import './styles.scss';
</script>

<div class="bg-[#f4eae1] min-h-[100vh]">
    <Header />
    <div class="min-h-[50vh] z-50 absolute w-full text-[#F2D6BE] text-4xl top-[10vh] drop-shadow-2xl">
        <div class="bg-[#674736] mx-[20%] rounded-3xl min-h-[80vh] p-4 border-2 border-[#F2D6BE]">
            <main>
                <slot />
            </main>
        </div>

        <footer>
            <!-- <p class="text-red-100">visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p> -->
        </footer>
    </div>

    <div class="static pt-[20vh]">
        <div class="flex-block text-center justify-center h-full">
            <div class="pt-4">
                <div>
                    <svg
                        class="waves"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        viewBox="0 24 150 28"
                        preserveAspectRatio="none"
                        shape-rendering="auto"
                    >
                        <defs>
                            <path
                                id="gentle-wave"
                                d="M-160 44c30 0 58-18 88-18s 58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z"
                            />
                        </defs>
                        <g class="parallax">
                            <use
                                xlink:href="#gentle-wave"
                                x="48"
                                y="7"
                                fill="#d0cec1"
                            />
                            <use
                                xlink:href="#gentle-wave"
                                x="48"
                                y="0"
                                fill="#F2D6BE"
                            />
                            <use
                                xlink:href="#gentle-wave"
                                x="48"
                                y="3"
                                fill="#A8806B"
                            />
                            <use
                                xlink:href="#gentle-wave"
                                x="48"
                                y="5"
                                fill="#553936"
                            />
                        </g>
                    </svg>
                </div>
            </div>
        </div>
    </div>
    <div class="bg-[#553936] min-h-[50vh]">
    </div>
</div>
