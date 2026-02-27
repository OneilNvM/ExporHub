import { supabase } from "../initClient";
import { Image } from "~/types/types";

export const downloadImages = async (images: Image[]): Promise<{signedUrl: string}[] | null> => {
    let imageArr = []

    try {
        if (images) {
            for (let i = 0; i < images?.length; i++) {
                const projectImage = images[i].file_path.split("/");

                console.log(projectImage[0], projectImage[1], projectImage[2])

                const { data, error } = await supabase.storage.from(projectImage[0]).createSignedUrl(`${projectImage[1]}/${projectImage[2]}`, 60)

                console.log(error?.stack)

                if (error) throw new Error(`Download error: ${error.message}`)

                imageArr.push(data)

                console.log(data)
            }
        }

        return imageArr
    } catch (error) {
        console.error(error)

        return null
    }

}