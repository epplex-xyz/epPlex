export async function readTextFileFromEndpoint(url: string): Promise<string[] | null> {
    try {
        const response = await fetch(url);
        if (response.ok) {
            const fileContent = await response.text();
            return fileContent.split('\n'); // Split content by line breaks
        } else {
            console.log(`Failed to fetch the file. Status code: ${response.status}`);
            return null;
        }
    } catch (error) {
        console.log("Error fetching the file:", error);
        return null;
    }
}
