use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page max-w-2xl mx-auto">
            <h1 class="text-2xl font-bold text-gray-900 mb-4">"เกี่ยวกับ PedCalc"</h1>
            <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 space-y-4 text-gray-700">
                <p>"PedCalc เป็นเครื่องมือคำนวณขนาดยาสำหรับเด็ก ออกแบบมาเพื่อช่วยบุคลากรทางการแพทย์ในการคำนวณขนาดยาอย่างรวดเร็วและแม่นยำ"</p>

                <div class="bg-amber-50 border border-amber-200 rounded-lg p-4">
                    <p class="font-medium text-amber-700">"⚠️ ข้อจำกัดความรับผิดชอบ"</p>
                    <p class="text-sm text-amber-600 mt-1">"เครื่องมือนี้ไม่ใช่การทดแทนความรู้ทางการแพทย์และการตัดสินใจทางคลินิก ผู้ใช้ต้องตรวจสอบความถูกต้องก่อนสั่งยาทุกครั้ง"</p>
                </div>

                <div>
                    <p class="font-semibold">"คุณสมบัติหลัก:"</p>
                    <ul class="list-disc list-inside text-sm mt-2 space-y-1">
                        <li>"คำนวณขนาดยาตามน้ำหนักและอายุ"</li>
                        <li>"รองรับยา 9 ชนิดที่ใช้บ่อย"</li>
                        <li>"คำเตือนและแจ้งเตือนทางคลินิก"</li>
                        <li>"แปลงหน่วย mg → mL อัตโนมัติ"</li>
                        <li>"ปรับขนาดยาตามภาวะไต/ตับ"</li>
                        <li>"บันทึกประวัติการคำนวณ (localStorage)"</li>
                    </ul>
                </div>

                <div>
                    <p class="font-semibold">"แหล่งอ้างอิง:"</p>
                    <ul class="list-disc list-inside text-sm mt-2 space-y-1">
                        <li>"BNF for Children 2023-2024"</li>
                        <li>"Harriet Lane Handbook, 22nd Edition"</li>
                        <li>"Thai Pediatric Formulary 2023"</li>
                        <li>"UpToDate Pediatric Drug Information"</li>
                    </ul>
                </div>

                <div class="pt-2 border-t border-gray-100 text-sm text-gray-500">
                    <p>"Drug Database Version: 2024.01"</p>
                    <p>"เทคโนโลยี: Rust + Leptos (WebAssembly)"</p>
                </div>
            </div>
        </div>
    }
}
