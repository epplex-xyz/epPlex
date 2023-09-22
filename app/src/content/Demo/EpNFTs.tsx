import React from "react";
import { Token22 } from "../../../client/types/token22";
import { EpNFTContainer } from "./EpNFTContainer";
import { Swiper, SwiperSlide } from "swiper/react";
import { EffectCoverflow, Navigation, Pagination } from "swiper/modules";

import 'swiper/css';
import 'swiper/css/effect-coverflow';
import 'swiper/css/pagination';
import 'swiper/css/navigation';
import "../../styles/swiper.module.scss";
// JG2sDKq9r3Q2HPzzJom6kXSuFZRB5LRFofW7f5xoCMy

// export function EpNFTs({items}: {items: Token22[]}) {
//     return (
//         <Swiper
//             effect={'coverflow'}
//             grabCursor={true}
//             centeredSlides={true}
//             loop={true}
//             slidesPerView={'auto'}
//             coverflowEffect={{
//                 rotate: 0,
//                 stretch: 0,
//                 depth: 100,
//                 modifier: 2.5,
//             }}
//             pagination={{ el: '.swiper-pagination', clickable: true }}
//             navigation={{
//                 nextEl: '.swiper-button-next',
//                 prevEl: '.swiper-button-prev',
//                 // clickable: true,
//             }}
//             modules={[EffectCoverflow, Pagination, Navigation]}
//             className="swiper_container"
//         >
//             <SwiperSlide>
//                 {items.map((t, i) =>
//                     <React.Fragment key={i}>
//                         <EpNFTContainer item={t}/>
//                     </React.Fragment>
//                 )}
//             </SwiperSlide>
//
//             <SwiperSlide>
//                 {items.map((t, i) =>
//                     <React.Fragment key={i}>
//                         <EpNFTContainer item={t}/>
//                     </React.Fragment>
//                 )}
//             </SwiperSlide>
//
//             <SwiperSlide>
//                 {items.map((t, i) =>
//                     <React.Fragment key={i}>
//                         <EpNFTContainer item={t}/>
//                     </React.Fragment>
//                 )}
//             </SwiperSlide>
//
//             <div className="slider-controler">
//                 <div className="swiper-button-prev slider-arrow">
//                     <ion-icon name="arrow-back-outline"></ion-icon>
//                 </div>
//                 <div className="swiper-button-next slider-arrow">
//                     <ion-icon name="arrow-forward-outline"></ion-icon>
//                 </div>
//                 <div className="swiper-pagination"></div>
//             </div>
//         </Swiper>
//     );
// }

export function EpNFTs({items}: {items: Token22[]}) {
    return (
        <>
            {items.map((t, i) =>
                <React.Fragment key={i}>
                    <EpNFTContainer item={t}/>
                </React.Fragment>
            )}
        </>
    );
}
