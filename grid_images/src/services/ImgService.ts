const BASE_URL =
  "https://pixabay.com/api/?key={insert_key}&";

  console.log(process.env);

export const ImageTypes = ["photo", "ilustation", "vector"];

export type TResult = {
  total: number;
  totalHits: number;
  hits: {
    id: number;
    pageURL: string;
    type: typeof ImageTypes;
    tags: string;

    previewURL: string;
    previewWidth: number;
    previewHeight: number;

    webformatURL: string;
    webformatWidth: number;
    webformatHeight: number;

    largeImageURL: string;
    fullHDURL: string;

    imageURL: string;
    imageWidth: number;
    imageHeight: number;
    imageSize: number;

    views: number;
    downloads: number;
    likes: number;
    comments: number;
    user_id: number;
    user: string;
    userImageURL: string;
  }[];
};

export default class ImgService {
  static async serachImages(searchParams: {
    /** search string */
    q: string;
  }) {
    const params = new URLSearchParams();
    params.set("q", searchParams.q);
    // params.set("id", "729509,729509");
    const url = BASE_URL + params.toString();
    try {
      const rawResponse = await fetch(url);
      return rawResponse.json();
    } catch (e) {
      console.error(e);
      return null;
    }
  }
}
