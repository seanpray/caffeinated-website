import { Client } from '@notionhq/client';

// we don't need any JS on this page, though we'll load
// it in dev so that we get hot module replacement
export const csr = true;

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = false;

const notion = new Client({ auth: import.meta.env.VITE_NOTION_API_KEY });
// console.log(import.meta.env.VITE_NOTION_API_KEY);

export interface _Post {
	title: string;
	date: string;
	text: string;
};

export const _loadPostMeta = async () => {
	const databaseId = import.meta.env.VITE_DB_ID;
	const response = await notion.databases.query({
		database_id: databaseId,
	});
	console.log(response);
};

// (async () => {
//   const blockId = '16d8004e5f6a42a6981151c22ddada12';
//   const response = await notion.blocks.children.list({
//     block_id: blockId,
//   });
//   console.log(response);
// })();
