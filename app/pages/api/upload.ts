import { NextApiRequest, NextApiResponse } from "next";
import { Drive } from "../../services/backend";

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse,
) {
    if (req.method === "POST") {
        const body = req.body;
        if (!body.name || !body.fileBuffer) {
            res.status(401).json({ message: "Empty body" });
        }

        const imgUrl = await Drive.uploadPng(body.name, body.fileBuffer);
        res.status(200).json({ message: imgUrl });
    } else {
        res.status(405).json({ message: "Only do POST" });
    }
}
