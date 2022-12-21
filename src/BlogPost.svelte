<script lang="ts">
    export let data;
    const formatDate = (unix_timestamp: number) => {
        const date = new Date(unix_timestamp * 1000);
        let hours = date.getHours();
        const pm = hours > 12;
        if (hours > 12) {
            hours -= 12;
        }
        return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}, ${hours}:${date.getMinutes()} ${pm ? "pm" : "am"}`
    };
</script>

<section class="mt-10 font-semibold">
	<div class="flex flex-wrap">
		<div class="ml-20 text-[#fcebda] text-6xl">
			<div class="ml-3" id={data.title}>
				{data.title}
                {#if !!data.author}
                    <div class="text-sm flex align-middle p-0.5">
                        <img class="h-8 w-8 rounded-full" src={data.author.pfp} alt="" />
                        <div class="align-middle ml-1 mt-1">
                            {data.author.name}
                        </div>
                    </div>
                {/if}
                {#if !!data.created_date}
                    <div class="text-sm">
                        created: {formatDate(data.created_date)}
                        {#if data.last_update}
                        - last updated: {formatDate(data.last_update)}
                        {/if}
                    </div>
                {/if}
			</div>
			<div class="flex dot-container">
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
			<div class="mt-12 ml-20 text-[#fcebda] text-5xl">
				{data.content}
			</div>
		</div>
	</div>
</section>

<style>
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
	@media (max-width: 1500px) {
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
