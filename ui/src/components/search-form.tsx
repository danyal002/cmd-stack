'use client';

import { Input } from '@/components/ui/input';
import { useSearch } from '@/use-command';
import { Search } from 'lucide-react';
import { startTransition } from 'react';

interface SearchFormProps {}

export function SearchForm({}: SearchFormProps) {
  const [_, setSearch] = useSearch();

  function onChange(e: React.ChangeEvent<HTMLInputElement>): void {
    const input = e.target.value;

    // avoid blocking the UI, this will trigger a refresh on the commands
    startTransition(() => {
        setSearch(input);
    })
  }

  return (
    <form>
      <div className="relative">
        <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input autoCapitalize='off' autoCorrect='off' onChange={onChange} placeholder="Search" className="pl-8" />
      </div>
    </form>
  );
}
