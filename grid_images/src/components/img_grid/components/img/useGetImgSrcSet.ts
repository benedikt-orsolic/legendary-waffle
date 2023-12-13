import { TResult } from "../../../../services/ImgService";

export default function useGetImgSourcSet(imgHit: TResult["hits"][0]) {
  const webFormatUrlSegments = imgHit.webformatURL.split("_640");

  const baseWebFormatUrl = webFormatUrlSegments[0];
  const webFormatExtention = webFormatUrlSegments[1].split(".")[1];
  const webFormatSizeRation = imgHit.webformatWidth / imgHit.webformatHeight;

  // Acording to docs 960 should also be available size but does not seem to work for me
  return [180, 340, 640].reduce<string>((acc, size) => {
    const url = baseWebFormatUrl + "_" + size + "." + webFormatExtention;
    let width = 0;

    if (webFormatSizeRation < 1) {
      width = size;
    } else {
      width = Math.round(size * webFormatSizeRation);
    }

    acc += `${url} ${width}w,`;
    return acc;
  }, "");
}
