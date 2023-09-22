import { NextApiRequest, NextApiResponse } from "next";


type Data = {
    message: string;
};

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    if (req.method === "POST") {

        const body = JSON.parse(req.body);

        // const tx =  await PROGRAM.createToken();
        // const admins = JSON.parse(process.env.ADMINS as string) as string[];
        // if (!verifySignature(req.body, admins)) {
        //     res.status(401).json({ message: "Invalid signature!" });
        // } else {
        //     res.status(200).json({ message: "Wallet verified" });
        // }
    } else {
        res.status(405).json({ message: "Only do POST" });
    }
}
