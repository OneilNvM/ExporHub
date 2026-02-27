import { supabase } from "../initClient";

export const uploadImages = async (images: FileList | null): Promise<void> => {
    try {
        if (images) {
            for (let i = 0; i < images?.length; i++) {
                const projectImage = images.item(i);

                const { data, error } = await supabase.storage.from('images').upload(`projects/${projectImage?.name}`, projectImage!)

                if (error) throw new Error(`Upload error: ${error.message}`)

                console.dir(data)
            }
        }
    } catch (error) {
        console.error(error)
    }

}