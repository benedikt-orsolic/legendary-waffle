import React from "react";

import ImgService, { TResult } from "../../services/ImgService";
import ImgGrid from "../../components/img_grid/ImgGrid";
import useDebouncedFn from "../../utils/useDebouncedFn";

export default function Home() {
  const [rawImgs, setRawImgs] = React.useState<TResult | null>(null);
  const debounceSearch = useDebouncedFn(async (searchValue: string) => {
    const data = await ImgService.serachImages({ q: searchValue });
    setRawImgs(data);
  });

  console.log(rawImgs);

  return (
    <>
      <h1>Home</h1>
      <input
        type="text"
        placeholder="Search"
        name="search"
        onChange={(e) => {
          debounceSearch(e.target.value);
        }}
      />
      <button
        onClick={async function fetchData() {
          const data = await ImgService.serachImages({ q: "flowers blue" });
          setRawImgs(data);
        }}
      >
        Get imgs
      </button>
      {rawImgs != null && <ImgGrid pageData={rawImgs} />}
    </>
  );
}
