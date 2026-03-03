use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-12 border-t border-gray-200 bg-white">
            <div class="max-w-6xl mx-auto px-4 py-6 text-center text-sm text-gray-500">
                <p class="font-medium text-gray-700">
                    "⚠️ คำเตือนสำคัญ"
                </p>
                <p class="mt-1">
                    "เครื่องมือนี้ออกแบบมาเพื่อช่วยในการคำนวณเท่านั้น ไม่ใช่การทดแทนการตัดสินใจทางคลินิก"
                </p>
                <p class="mt-1">
                    "แพทย์และผู้สั่งยาต้องตรวจสอบความถูกต้องของขนาดยาทุกครั้งก่อนการสั่งและให้ยา"
                </p>
                <p class="mt-3 text-xs">
                    "Drug DB v2024.01 | อ้างอิง: BNFC 2023-2024, Harriet Lane 22nd ed., Thai Pediatric Formulary 2023"
                </p>
            </div>
        </footer>
    }
}
