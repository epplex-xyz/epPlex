import React from "react";
import { Text } from "@components/Text/TextComponent";
import { CopyTooltip } from "@components/Tooltip/MyTooltip";
import { spliceAddress } from "../../../utils/general";

function AddressCopy({ address }: { address: string }) {
    return (
        <div className="flex gap-x-1 items-center">
            <Text.Body2>{spliceAddress(address)}</Text.Body2>

            <CopyTooltip copyText={address} />
        </div>
    );
}
