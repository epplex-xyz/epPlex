import { Section } from "src/components/Container/Section";
import { BoxProps } from "@mui/material/Box";
import { Text } from "src/components/Text/TextComponent";
import { ButtonLink } from "src/components/Buttons/LinkButton";
import { ButtonConfig } from "src/components/Buttons/ButtonConfig";

function LandingText(){
    return (
        <div className={"flex flex-col text-center items-center gap-y-4"}>
            <Text.H1>
                epPlex
            </Text.H1>
            <Text.H3>
                Empowering ephemeral NFTs on Solana
            </Text.H3>
        </div>
    );
}


export function LandingPage({...props}: BoxProps){
    return (
        <Section {...props}>
            <div className={"flex flex-col items-center gap-y-8"}>
                <LandingText/>

                <div className={"flex gap-x-6"}>
                    <ButtonLink {...ButtonConfig.demo}/>
                    <ButtonLink {...ButtonConfig.docs}/>
                </div>
            </div>
        </Section>
    );
}