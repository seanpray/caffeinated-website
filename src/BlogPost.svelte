<script lang="ts">
    export let data;
    export let last;
    const formatDate = (unix_timestamp: number) => {
        const date = new Date(unix_timestamp * 1000);
        let hours = date.getHours();
        const pm = hours > 12;
        if (hours > 12) {
            hours -= 12;
        }
        return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}, ${hours}:${date.getMinutes()} ${pm ? "pm" : "am"}`
    };
    let content = [];
    let links = [];
    let current_content = "";
    let current_link = "";
    let image_open = false;
    let first = false;
    // WHY DOES NOTION INSERT AN EXTRA SPACE IN BETWEEN LINKS
    let next = false;
    let next2 = false;
    let image_flag = false;
    for (let i = 0; i < data.content.length; i++) {
        const c = data.content[i];
        if (c == "<") {
            first = true;
            image_open = true;
        } else if (c == ">") {
            if (image_flag) {
                links.push(current_link);
                current_link = "";
                image_flag = false;
                continue;
            }
            image_open = false;
        }
        if (!image_open && !image_flag) {
            current_content += c;
            continue;
        }
        // if it's not http/https, then we consider invalid
        if (first && !image_flag) {
            next = true;
            first = false;
            current_content += c;
            continue;
        } else if (next && c == " " && !image_flag) {
            next = false;
            next2 = true;
            continue;
        } else if (next2 && c == "h" && !image_flag) {
            current_content = current_content.slice(0, -1);
            image_flag = true;
        } else if (!image_flag) {
            current_content += c;
            continue;
        }
        next2 = false;
        if (current_content.length > 0) {
            content.push(current_content);
            current_content = "";
        }
        current_link += c;
    }
    content.push(current_content);
    links.push(current_link);
</script>

<section class="mt-10 font-semibold">
	<div class="flex flex-wrap">
		<div class="ml-20 text-[#fcebda] text-6xl">
			<div class="ml-3" id={data.title}>
				{data.title}
                <!-- {#if !!data.author} -->
                <!--     <div class="text-sm flex align-middle p-0.5"> -->
                <!--         <img class="h-8 w-8 rounded-full" src={data.author.pfp} alt="" /> -->
                <!--         <div class="align-middle ml-1 mt-1"> -->
                <!--             {data.author.name} -->
                <!--         </div> -->
                <!--     </div> -->
                <!-- {/if} -->
                {#if !!data.created_date}
                    <div class="text-sm">
                        created: {formatDate(data.created_date)}
                        {#if data.last_update}
                        - last updated: {formatDate(data.last_update)}
                        {/if}
                    </div>
                {/if}
			</div>
            <div class="sep-line" />
			<div class="mt-12 ml-2 text-[#fcebda] text-5xl max-w-[53vw]">
                {#each content as p, i}
                    {p}
                    <img class="max-w-[53vw]" src={links[i]} alt="" />
                {/each}
			</div>
            {#if !last}
                <div class="flex dot-container mt-24">
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
                    <div class="dot2" />
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
    .sep-line {
        width: 40rem;
        border-bottom: 1px solid #bc8f62;
        margin-top: 1.1rem;
    }
	.dot-container {
		width: 40rem;
	}
	.dot2 {
		height: 1.1rem;
		width: 1.1rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 100;
		border-radius: 50%;
		margin: 0.7rem;
	}
	@media (max-width: 1100px) {
        .sep-line {
            width: 60vw;
            border-bottom: 1px solid #bc8f62;
            margin-top: 1.1rem;
        }
		.dot2 {
			height: 1vw;
			width: 1vw;
			border: 1px solid #bc8f62;
			background: #bc8f62;
			z-index: 100;
			border-radius: 50%;
			margin: 0.8vw;
		}
		.dot-container {
			width: 50vw;
		}
	}
</style>
