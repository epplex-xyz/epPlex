
interface SectionConfigType {
    id: string;
}

type SectionConfigKey = "landingPage";

export const SectionConfig: Record<SectionConfigKey, SectionConfigType> = {
    landingPage: {
        id: "landingPageId"
    },
};