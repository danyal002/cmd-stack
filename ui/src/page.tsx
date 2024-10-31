import { Mail } from "@/components/mail"
import { accounts, mails } from "@/data"

export default function MailPage() {
  const defaultLayout = undefined
  const defaultCollapsed = undefined
  
  return (
    <>
      <div className="hidden flex-col md:flex">
        <Mail
          accounts={accounts}
          mails={mails}
          defaultLayout={defaultLayout}
          defaultCollapsed={defaultCollapsed}
          navCollapsedSize={4}
        />
      </div>
    </>
  )
}
