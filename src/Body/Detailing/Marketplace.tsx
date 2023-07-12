import { ToggleButtonGroup, ToggleButton } from "@mui/material";
import { SPaper } from "../../Styled";
import { marketplaces } from "../../enums";

interface Props {
    marketplace: marketplaces;
    setMarketplace: React.Dispatch<React.SetStateAction<marketplaces>>;
}

function Marketplace({ marketplace, setMarketplace }: Props) {
    const handleChange = (
        event: React.MouseEvent<HTMLElement>,
        newMarketplace: marketplaces | null,
    ) => {
        if (newMarketplace !== null) {
            setMarketplace(newMarketplace);
        }
    };

    return (
        <SPaper>
            <ToggleButtonGroup
                value={marketplace}
                exclusive
                onChange={handleChange}
                aria-label="Выбор магазина"
            >
                <ToggleButton value={marketplaces.Ozon} aria-label="Ozon">
                    Ozon
                </ToggleButton>
                <ToggleButton value={marketplaces.Wildberries} aria-label="Wildberries">
                    Wildberries
                </ToggleButton>
                <ToggleButton value={marketplaces.Yandex} aria-label="Yandex">
                    Yandex
                </ToggleButton>
            </ToggleButtonGroup>
        </SPaper>
    );
}

export default Marketplace;
