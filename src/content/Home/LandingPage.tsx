import { Section } from "@components/Container/Section";
import { BoxProps } from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import { ButtonLink } from "@components/Buttons/LinkButton";
import { ButtonConfig } from "@components/Buttons/ButtonConfig";

// Could use react type animation for the crypto wod
function LandingText(){
    return (
        <div className={"flex flex-col text-center items-center gap-y-4"}>
            <Text.H1>
                Test
            </Text.H1>
            <Text.H1>
                Testing
            </Text.H1>

            <div className={"w-[650px]"}>
                <Text.Body1>
                    Test text
                </Text.Body1>
            </div>
        </div>
    );
}

// Not sure what positioning this is supposed to be
function AvailableOn() {
    return (
        <div className={"absolute top-0 left-0"}>
            <Text.Body1>
                 Available on
            </Text.Body1>
            {/*  Insert blockchain icons  */}
        </div>
    );
}

export function LandingPage({...props}: BoxProps){
    return (
        <Section {...props}>
            <AvailableOn/>

            <div className={"flex flex-col items-center gap-y-6"}>
                <div><ButtonLink {...ButtonConfig.access}/></div>

                <LandingText/>

                <div className={"flex gap-x-6"}>
                    <ButtonLink {...ButtonConfig.app}/>
                    <ButtonLink {...ButtonConfig.docs}/>
                </div>
            </div>
        </Section>
    );
}